use core::time;

use poise::serenity_prelude::{self as serenity, Mentionable, UserId};

use crate::config::Config;

pub struct EmbedOptions<'a> {
	pub user: &'a serenity::User,
	pub content: &'a str,
	pub outgoing: bool,
	pub anonymous: bool,
	pub details: bool,
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
			serenity::CreateEmbedAuthor::new(&config.anonymous_display_name).icon_url(
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

	if options.details {
		result = result.footer(serenity::CreateEmbedFooter::new(format!(
			"Username: {} ({})",
			options.user.tag(),
			options.user.id,
		)));
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
	user_id: UserId,
	actor_id: UserId,
	timestamp: serenity::Timestamp,
) -> String {
	let discord_timestamp = serenity::FormattedTimestamp::new(
		timestamp,
		Some(serenity::FormattedTimestampStyle::RelativeTime),
	);

	let opened_message = if user_id == actor_id {
		format!(
			"📩 Thread opened by {} {}.",
			user_id.mention(),
			discord_timestamp
		)
	} else {
		format!(
			"📩 Thread for {} opened by {} {}",
			user_id.mention(),
			actor_id.mention(),
			discord_timestamp
		)
	};

	if let Some(role) = config.mention_role {
		format!("{}: {}", role.mention(), opened_message)
	} else {
		opened_message
	}
}
