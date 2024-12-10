use poise::serenity_prelude as serenity;

pub fn message_as_embed(message: &serenity::Message) -> serenity::CreateEmbed {
	message_as_embed_raw(&message.author, &message.content, &message.attachments)
}

pub fn message_as_embed_raw(author: &serenity::User, content: &str, attachments: &[serenity::Attachment]) -> serenity::CreateEmbed {
	serenity::CreateEmbed::new()
		.author(
			serenity::CreateEmbedAuthor::new(author.display_name()).icon_url(
				author
					.avatar_url()
					.unwrap_or_else(|| author.default_avatar_url()),
			).url(format!("https://discord.com/users/{}", author.id)),
		)
		.description(content)
		.footer(serenity::CreateEmbedFooter::new(format!("User Tag: {}", author.tag())))
}