use poise::serenity_prelude as serenity;

use crate::data::sent_messages;
use crate::data::sent_messages::SentMessage;
use crate::data::tags;
use crate::data::threads;
use crate::formatting::message_embed;
use crate::util::attachments::{clone_attachment, first_image_attachment};
use crate::util::json_error_codes::get_json_error_code;
use crate::util::json_error_codes::CANNOT_MESSAGE;
use crate::util::markdown;

use super::util::Context;
use super::util::{complete_tags, require_staff};

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
	// TODO: add a limit
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

/// Reply to a mod-mail thread with a tag.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("tr", "treply")
)]
pub async fn tag_reply(
	context: Context<'_>,
	#[rest]
	#[description = "The tag name."]
	#[autocomplete = "complete_tags"]
	name: String,
) -> eyre::Result<()> {
	create_tag(context, &name, false).await
}

/// Reply to a mod-mail thread with a tag anonymously.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("atr", "atag_reply", "atreply")
)]
pub async fn anon_tag_reply(
	context: Context<'_>,
	#[rest]
	#[description = "The tag name."]
	name: String,
) -> eyre::Result<()> {
	create_tag(context, &name, true).await
}

async fn create_tag(context: Context<'_>, name: &str, anonymous: bool) -> eyre::Result<()> {
	let tag = tags::get(&context.data().pg, name).await?;

	match tag {
		Some(message) => {
			create(context, &message, anonymous).await?;
		}
		None => {
			context
				.send(
					poise::CreateReply::default()
						.content(format!(
							"❌ Tag named **{}** cannot be found.",
							markdown::escape(name)
						))
						.ephemeral(true),
				)
				.await?;
		}
	}

	Ok(())
}

async fn create(context: Context<'_>, message: &str, anonymous: bool) -> eyre::Result<()> {
	context.defer().await?;

	let Some(thread_data) = threads::get(&context.data().pg, context.channel_id().get()).await?
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

	let dm_channel = serenity::ChannelId::new(thread_data.dm_channel_id);

	let first_image_attachment = if let Context::Prefix(context) = context {
		first_image_attachment(&context.msg.attachments)
	} else {
		None
	};

	let forwarded_attachments = if let Some(image_attachment) = first_image_attachment {
		let cloned = clone_attachment(context.http(), image_attachment).await?;
		vec![cloned]
	} else {
		vec![]
	};

	let forwarded_image_filename = first_image_attachment
		.as_ref()
		.map(|attachment| attachment.filename.clone());

	let forwarded_message_builder =
		serenity::CreateMessage::new().add_embed(message_embed::create(
			context.serenity_context(),
			&context.data().config,
			message_embed::Options {
				author: context.author(),
				content: message,
				image_filename: forwarded_image_filename.as_deref(),
				outgoing: false,
				anonymous,
				user_info: false,
			},
		));

	let forwarded_message_result = dm_channel
		.send_files(
			&context,
			forwarded_attachments.clone(),
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
			image_filename: forwarded_image_filename.as_deref(),
			outgoing: true,
			anonymous,
			user_info: true,
		},
	));

	source_message_builder.attachments = forwarded_attachments;

	let source_message_handle = context.send(source_message_builder).await?;

	sent_messages::insert(
		&context.data().pg,
		SentMessage {
			id: source_message_handle.message().await?.id.get(),
			thread_id: context.channel_id().get(),
			forwarded_message_id: forwarded_message.id.get(),
			author_id: context.author().id.get(),
			anonymous,
			image_filename: forwarded_image_filename,
		},
	)
	.await?;

	if let poise::Context::Prefix(prefix) = context {
		prefix.msg.delete(&context.http()).await.ok();
	}

	Ok(())
}
