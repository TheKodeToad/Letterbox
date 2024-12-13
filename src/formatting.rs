use poise::serenity_prelude as serenity;

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
