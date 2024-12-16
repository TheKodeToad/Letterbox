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

pub fn get_json_error_code(error: &serenity::Error) -> Option<isize> {
	if let serenity::Error::Http(serenity::HttpError::UnsuccessfulRequest(
		serenity::ErrorResponse {
			error: serenity::DiscordJsonError { code, .. },
			..
		},
	)) = error {
		Some(*code)
	} else {
		None
	}
}

// hacky but it works pretty well
// can't find any crates for this 🤷‍♀️
const FORMATTING_CHARS: &str = r#"\/*_-`#@<>.~|:[]()"#;

pub fn escape_markdown(input: &str) -> String {
	let mut result = String::new();

	for char in input.chars() {
		if FORMATTING_CHARS.contains(char) {
			result.push('\\');
		}

		result.push(char);
	}

	result
}
