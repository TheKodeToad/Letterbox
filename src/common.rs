use poise::serenity_prelude::{self as serenity, CreateEmbedAuthor};

pub fn message_as_embed(message: &serenity::Message) -> serenity::CreateEmbed {
	serenity::CreateEmbed::new()
		.author(
			CreateEmbedAuthor::new(message.author.tag()).icon_url(
				message
					.author
					.avatar_url()
					.unwrap_or_else(|| message.author.default_avatar_url()),
			),
		)
		.description(&message.content)
}
