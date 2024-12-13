use poise::serenity_prelude::{self as serenity, CreateEmbedAuthor, User};

use crate::config::Config;

pub fn make_embed(
	context: &serenity::Context,
	config: &Config,
	user: &serenity::User,
	content: &str,
	outgoing: bool,
	anonymous: bool,
	details: bool,
) -> serenity::CreateEmbed {
	let mut result = serenity::CreateEmbed::new().description(content);

	if outgoing {
		result = result.color(serenity::colours::branding::GREEN);
	} else {
		result = result.color(serenity::colours::branding::YELLOW);
	}

	if anonymous {
		result = result.author(
			serenity::CreateEmbedAuthor::new("Staff Team").icon_url(
				config
					.server_id
					.to_guild_cached(&context.cache)
					.and_then(|guild| guild.icon_url())
					.unwrap_or_default(),
			),
		);
	} else {
		result = result.author(
			serenity::CreateEmbedAuthor::new(user.display_name()).icon_url(
				user.avatar_url()
					.unwrap_or_else(|| user.default_avatar_url()),
			),
		);
	}

	if details {
		result = result.footer(serenity::CreateEmbedFooter::new(format!(
			"Username: {} ({})",
			user.tag(),
			user.id
		)));
	}

	result
}
