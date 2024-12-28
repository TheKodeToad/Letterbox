use poise::serenity_prelude as serenity;

use crate::config::Config;

pub struct MessageEmbedOptions<'a> {
	pub author: &'a serenity::User,
	pub content: &'a str,
	pub image_filename: Option<&'a str>,
	pub outgoing: bool,
	pub anonymous: bool,
	pub user_info: bool,
}

pub fn make_message_embed(
	context: &serenity::Context,
	config: &Config,
	options: MessageEmbedOptions,
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
			serenity::CreateEmbedAuthor::new(options.author.display_name()).icon_url(
				options
					.author
					.avatar_url()
					.unwrap_or_else(|| options.author.default_avatar_url()),
			),
		);
	}

	if options.user_info {
		result = result.footer(serenity::CreateEmbedFooter::new(format!(
			"Username: {} ({})",
			options.author.tag(),
			options.author.id,
		)));
	}

	if let Some(filename) = options.image_filename {
		result = result.attachment(filename);
	}

	result
}

