use poise::serenity_prelude as serenity;

use crate::{
	data::{
		blocked_users::is_user_blocked,
		received_messages::{insert_received_message, ReceivedMessage},
		threads::{delete_thread, get_thread_by_dm_channel, insert_thread, Thread},
	},
	formatting::{fake_snapshot, message_embed, thread_info, user_info_embed},
	util::{
		attachments::{clone_attachment, first_image_attachment},
		json_error_codes::{get_json_error_code, UNKNOWN_CHANNEL},
	},
	Data,
};

pub async fn handle(
	context: &serenity::Context,
	message: &serenity::Message,
	data: &Data,
) -> eyre::Result<()> {
	let result = handle_impl(context, message, data).await;

	if let Err(error) = result {
		message
			.react(&context, serenity::ReactionType::Unicode("❌".to_string()))
			.await
			.ok();

		return Err(error);
	}

	Ok(())
}

async fn handle_impl(
	context: &serenity::Context,
	message: &serenity::Message,
	data: &Data,
) -> eyre::Result<()> {
	// if discord introduces another type of channel this could break :(
	if message.guild_id.is_some() {
		return Ok(());
	}

	if message.author.bot {
		return Ok(());
	}

	if !(message.kind == serenity::MessageType::Regular
		|| message.kind == serenity::MessageType::InlineReply)
	{
		return Ok(());
	}

	let existing_thread = get_thread_by_dm_channel(&data.pg, message.channel_id.get()).await?;

	let mut thread = if let Some(existing_thread) = existing_thread {
		serenity::ChannelId::new(existing_thread.id)
	} else {
		if is_user_blocked(&data.pg, message.author.id.get()).await? {
			return Ok(());
		}

		create_thread_from(context, message, data).await?
	};

	let snapshot = fake_snapshot::create(context, message, data.config.server_id);

	let image_attachment = message.message_snapshots.first().map_or_else(
		|| first_image_attachment(&message.attachments),
		|snapshot| first_image_attachment(&snapshot.attachments),
	);
	let image_filename = image_attachment.map(|attachment| attachment.filename.clone());
	let cloned_image_attachment = if let Some(attachment) = image_attachment {
		Some(clone_attachment(&context.http, attachment).await?)
	} else {
		None
	};

	let forwarded_message_builder =
		serenity::CreateMessage::new().add_embed(message_embed::create(
			context,
			&data.config,
			message_embed::Options {
				author: &message.author,
				content: snapshot.as_ref().unwrap_or(&message.content),
				image_filename: image_filename.as_deref(),
				outgoing: false,
				anonymous: false,
				user_info: true,
			},
		));

	let files = cloned_image_attachment
		.map(|attachment| vec![attachment])
		.unwrap_or_default();

	let forwarded_message_result = thread
		.send_files(context, files.clone(), forwarded_message_builder.clone())
		.await;

	let forwarded_message = match forwarded_message_result {
		Ok(forwarded_message) => forwarded_message,
		Err(err) => {
			// matching all errors could result in a thread being erroneosly deleted (which is irreversible)
			if let Some(UNKNOWN_CHANNEL) = get_json_error_code(&err) {
				if is_user_blocked(&data.pg, message.author.id.get()).await? {
					return Ok(());
				}

				delete_thread(&data.pg, thread.get()).await?;

				thread = create_thread_from(context, message, data).await?;

				thread
					.send_files(context, files, forwarded_message_builder)
					.await?
			} else {
				return Err(err.into());
			}
		}
	};

	insert_received_message(
		&data.pg,
		ReceivedMessage {
			id: message.id.get(),
			thread_id: thread.get(),
			forwarded_message_id: forwarded_message.id.get(),
			image_filename,
		},
	)
	.await?;

	message
		.react(
			&context.http,
			serenity::ReactionType::Unicode("✅".to_string()),
		)
		.await?;

	Ok(())
}

async fn create_thread_from(
	context: &serenity::Context,
	message: &serenity::Message,
	data: &Data,
) -> eyre::Result<serenity::ChannelId> {
	let created_at = message.id.created_at();
	let info_builder = serenity::CreateMessage::new()
		.content(thread_info::create(
			&data.config,
			thread_info::Options {
				user_id: message.author.id,
				opened: (message.author.id, created_at),
				closed: None,
			},
		))
		.allowed_mentions(thread_info::create_allowed_mentions(&data.config))
		.embed(user_info_embed::create(context, &data.config, &message.author).await?);
	let mut forum_post_builder = serenity::CreateForumPost::new(
		format!("Thread from {}", &message.author.tag()),
		info_builder,
	);
	if let Some(open_tag_id) = data.config.forum_channel.open_tag_id {
		forum_post_builder = forum_post_builder.add_applied_tag(open_tag_id);
	}
	let forum_post = data
		.config
		.forum_channel
		.id
		.create_forum_post(&context.http, forum_post_builder)
		.await?;
	insert_thread(
		&data.pg,
		Thread {
			id: forum_post.id.get(),
			dm_channel_id: message.channel_id.get(),
			user_id: message.author.id.get(),
			opened_by_id: message.author.id.get(),
			created_at: *created_at,
		},
	)
	.await?;

	let mut thread_open_message = "🧵 Started a new thread.".to_string();

	if let Some(suffix) = &data.config.messages.thread_open {
		thread_open_message.push('\n');
		thread_open_message += suffix;
	}

	message
		.channel_id
		.send_message(
			&context.http,
			serenity::CreateMessage::new().content(thread_open_message),
		)
		.await?;

	Ok(forum_post.id)
}
