use poise::serenity_prelude as serenity;

pub fn first_image_attachment(
	attachments: &[serenity::Attachment],
) -> Option<&serenity::Attachment> {
	attachments.iter().find(|attachment| {
		attachment
			.content_type
			.as_ref()
			.is_some_and(|content_type| content_type.starts_with("image/"))
	})
}

/// Clone an attachment by reuploading it.
pub async fn clone_attachment(
	http: &serenity::Http,
	attachment: &serenity::Attachment,
) -> eyre::Result<serenity::CreateAttachment> {
	let mut result = serenity::CreateAttachment::url(&http, &attachment.url).await?;
	result.filename.clone_from(&attachment.filename);

	Ok(result)
}
