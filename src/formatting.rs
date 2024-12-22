use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::config::Config;

pub struct EmbedOptions<'a> {
	pub user: &'a serenity::User,
	pub content: &'a str,
	pub image_filename: Option<&'a str>,
	pub outgoing: bool,
	pub anonymous: bool,
	pub user_info: bool,
}

pub fn make_message_embed(
	context: &serenity::Context,
	config: &Config,
	options: &EmbedOptions,
) -> serenity::CreateEmbed {
	let mut result = serenity::CreateEmbed::new().description(options.content);

	if options.outgoing {
		result = result.color(serenity::colours::branding::GREEN);
	} else {
		result = result.color(serenity::colours::branding::YELLOW);
	}

	if options.anonymous {
		result = result.author(
			serenity::CreateEmbedAuthor::new(&config.messages.anonymous_reply_title).icon_url(
				config
					.server_id
					.to_guild_cached(&context.cache)
					.and_then(|guild| guild.icon_url())
					.unwrap_or_default(),
			),
		);
	} else {
		result = result.author(
			serenity::CreateEmbedAuthor::new(options.user.display_name()).icon_url(
				options
					.user
					.avatar_url()
					.unwrap_or_else(|| options.user.default_avatar_url()),
			),
		);
	}

	if options.user_info {
		result = result.footer(serenity::CreateEmbedFooter::new(format!(
			"Username: {} ({})",
			options.user.tag(),
			options.user.id,
		)));
	}

	if let Some(filename) = options.image_filename {
		result = result.attachment(filename);
	}

	result
}

pub async fn make_info_embed(
	context: &serenity::Context,
	config: &Config,
	user: &serenity::User,
) -> eyre::Result<serenity::CreateEmbed> {
	let member = config.server_id.member(context, user.id).await.ok();

	let mut result = serenity::CreateEmbed::new()
		.author(
			serenity::CreateEmbedAuthor::new(user.display_name()).icon_url(
				user.avatar_url()
					.unwrap_or_else(|| user.default_avatar_url()),
			),
		)
		.footer(serenity::CreateEmbedFooter::new(format!(
			"Username: {} ({})",
			user.tag(),
			user.id,
		)))
		.color(serenity::colours::branding::BLURPLE);

	let created_at_text = serenity::FormattedTimestamp::new(
		user.created_at(),
		Some(serenity::FormattedTimestampStyle::ShortDateTime),
	)
	.to_string();

	result = result.field("Account Created At", created_at_text, true);

	if let Some(member) = member {
		let joined_at_text = member
			.joined_at
			.map(|joined_at| {
				serenity::FormattedTimestamp::new(
					joined_at,
					Some(serenity::FormattedTimestampStyle::ShortDateTime),
				)
				.to_string()
			})
			.unwrap_or("*Unknown Date*".to_string());

		result = result.field("Server Member Since", joined_at_text, true);

		if let Some(roles) = member.roles(&context.cache) {
			let roles_text = if roles.is_empty() {
				"*No Roles*".to_string()
			} else {
				roles
					.iter()
					.map(|role| role.mention().to_string())
					.collect::<Vec<String>>()
					.join(" ")
			};

			result = result.field("Roles", roles_text, false);
		}
	}

	Ok(result)
}

pub fn make_info_content(
	config: &Config,
	user_id: serenity::UserId,
	opened_by_id: serenity::UserId,
	opened_timestamp: serenity::Timestamp,
	closed_by_id: Option<serenity::UserId>,
	closed_timestamp: Option<serenity::Timestamp>,
) -> String {
	let opened_discord_timestamp = serenity::FormattedTimestamp::new(
		opened_timestamp,
		Some(serenity::FormattedTimestampStyle::RelativeTime),
	);

	let mut result = String::new();

	if let Some(role) = config.forum_channel.mention_role_id {
		result += &role.mention().to_string();
		result += "\n\n";
	}

	if user_id == opened_by_id {
		result += &format!(
			"📩 Thread opened by {} {}",
			user_id.mention(),
			opened_discord_timestamp
		);
	} else {
		result += &format!(
			"📩 Thread for {} opened by {} {}",
			user_id.mention(),
			opened_by_id.mention(),
			opened_discord_timestamp
		);
	}

	if closed_by_id.is_some() && closed_timestamp.is_some() {
		let closed_discord_timestamp = serenity::FormattedTimestamp::new(
			closed_timestamp.unwrap(),
			Some(serenity::FormattedTimestampStyle::RelativeTime),
		);

		result += &format!(
			"\n⛔ Thread closed by {} {}",
			serenity::Mention::User(closed_by_id.unwrap()),
			closed_discord_timestamp
		);
	}

	result
}
