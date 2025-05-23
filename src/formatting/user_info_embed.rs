use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::config::Config;

pub async fn create(
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

	if let Some(member) = &member {
		if let Some(roles) = member.roles(&context.cache) {
			if !roles.is_empty() {
				let mut sorted_roles = roles.clone();
				sorted_roles.sort_by(|a, b| b.position.cmp(&a.position));

				let roles_text = sorted_roles
					.iter()
					.map(|role| role.mention().to_string())
					.collect::<Vec<String>>()
					.join(" ");

				result = result.field("Roles", roles_text, false);
			}
		}
	}

	let created_at_text = serenity::FormattedTimestamp::new(
		user.created_at(),
		Some(serenity::FormattedTimestampStyle::ShortDateTime),
	)
	.to_string();

	result = result.field("Account Created At", created_at_text, true);

	if let Some(member) = &member {
		let joined_at_text = member
			.joined_at
			.map_or("*Unknown Date*".to_string(), |joined_at| {
				serenity::FormattedTimestamp::new(
					joined_at,
					Some(serenity::FormattedTimestampStyle::ShortDateTime),
				)
				.to_string()
			});

		result = result.field("Server Member Since", joined_at_text, true);
	}

	Ok(result)
}
