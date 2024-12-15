use poise::serenity_prelude::{self as serenity, CreateAttachment};

pub fn first_image_attachment(
	attachments: &[serenity::Attachment],
) -> Option<&serenity::Attachment> {
	attachments
		.iter()
		.find(|attachment| {
			attachment
				.content_type
				.as_ref()
				.map(|content_type| content_type.starts_with("image/"))
				.unwrap_or_default()
		})
}

/// Clone an attachment by reuploading it.
pub async fn clone_attachment(
	http: &serenity::Http,
	attachment: &serenity::Attachment,
) -> eyre::Result<CreateAttachment> {
	let mut result = serenity::CreateAttachment::url(&http, &attachment.url).await?;
	result.filename = attachment.filename.clone();

	Ok(result)
}
