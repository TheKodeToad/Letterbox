use poise::serenity_prelude as serenity;

use crate::data::sent_messages::insert_sent_message;
use crate::data::sent_messages::SentMessage;
use crate::data::threads::get_thread;
use crate::formatting::message_embed;
use crate::util::attachments::first_image_attachment;
use crate::util::json_error_codes::get_json_error_code;
use crate::util::json_error_codes::CANNOT_MESSAGE;

use super::util::require_staff;
use super::util::Context;

const CANNOT_MESSAGE_ERROR: &str =
	"❌ Cannot currently send messages to the user. This is most likely because:
- The app has been blocked.
- The user does not share any mutual servers.
- The users privacy settings do not allow direct messages.";

/// Reply to a mod-mail thread.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("r")
)]
pub async fn reply(
	context: Context<'_>,
	#[rest]
	#[description = "The message to send."]
	message: String,
) -> eyre::Result<()> {
	create(context, &message, false).await
}

/// Reply to a mod-mail thread anonymously.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("ar", "areply")
)]
pub async fn anon_reply(
	context: Context<'_>,
	#[rest]
	#[description = "The message to send."]
	message: String,
) -> eyre::Result<()> {
	create(context, &message, true).await
}

async fn create(context: Context<'_>, message: &str, anonymous: bool) -> eyre::Result<()> {
	let Some(thread_data) = get_thread(&context.data().pg, context.channel_id().get()).await?
	else {
		context
			.send(
				poise::CreateReply::default()
					.content("❌ No open thread in this channel.")
					.ephemeral(true),
			)
			.await?;
		return Ok(());
	};

	context.defer().await?;

	let dm_channel = serenity::ChannelId::new(thread_data.dm_channel_id);

	let image_attachment = if let Context::Prefix(context) = context {
		first_image_attachment(&context.msg.attachments)
	} else {
		None
	};

	let image_attachment_clone = if let Some(image_attachment) = image_attachment {
		let mut forwarded_attachment =
			serenity::CreateAttachment::url(&context.http(), &image_attachment.url).await?;
		forwarded_attachment
			.filename
			.clone_from(&image_attachment.filename);

		Some(forwarded_attachment)
	} else {
		None
	};

	let image_filename = image_attachment
		.as_ref()
		.map(|attachment| attachment.filename.clone());

	let forwarded_message_builder =
		serenity::CreateMessage::new().add_embed(message_embed::create(
			context.serenity_context(),
			&context.data().config,
			message_embed::Options {
				author: context.author(),
				content: message,
				image_filename: image_filename.as_deref(),
				outgoing: false,
				anonymous,
				user_info: false,
			},
		));

	let forwarded_message_result = dm_channel
		.send_files(
			&context,
			image_attachment_clone
				.as_ref()
				.map(|attachment| vec![attachment.clone()])
				.unwrap_or_default(),
			forwarded_message_builder,
		)
		.await;

	let forwarded_message = match forwarded_message_result {
		Ok(forwarded_message) => forwarded_message,
		Err(error) => {
			if let Some(CANNOT_MESSAGE) = get_json_error_code(&error) {
				context.say(CANNOT_MESSAGE_ERROR).await?;
				return Ok(());
			}
			return Err(error.into());
		}
	};

	let mut source_message_builder = poise::CreateReply::default().embed(message_embed::create(
		context.serenity_context(),
		&context.data().config,
		message_embed::Options {
			author: context.author(),
			content: message,
			image_filename: image_filename.as_deref(),
			outgoing: true,
			anonymous,
			user_info: true,
		},
	));

	if let Some(image_attachment_clone) = image_attachment_clone {
		source_message_builder = source_message_builder.attachment(image_attachment_clone);
	}

	let source_message_handle = context.send(source_message_builder).await?;

	insert_sent_message(
		&context.data().pg,
		SentMessage {
			id: source_message_handle.message().await?.id.get(),
			thread_id: context.channel_id().get(),
			forwarded_message_id: forwarded_message.id.get(),
			author_id: context.author().id.get(),
			anonymous,
			image_filename,
		},
	)
	.await?;

	if let poise::Context::Prefix(prefix) = context {
		prefix.msg.delete(&context.http()).await.ok();
	}

	Ok(())
}
