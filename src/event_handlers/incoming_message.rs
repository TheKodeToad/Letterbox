use poise::serenity_prelude as serenity;

use crate::{
	data::{
		blocked_users::is_user_blocked, received_messages::{insert_received_message, ReceivedMessage}, threads::{get_thread_by_dm_channel, insert_thread, Thread}
	},
	formatting::{make_info_content, make_info_embed, make_message_embed, EmbedOptions},
	util::{clone_attachment, first_image_attachment},
	Data,
};

pub async fn handle_incoming_message(
	context: &serenity::Context,
	message: &serenity::Message,
	data: &Data,
) -> eyre::Result<()> {
	let result = handle_incoming_message_impl(context, message, data).await;

	if let Err(error) = result {
		message
			.react(&context, serenity::ReactionType::Unicode("❌".to_string()))
			.await
			.ok();

		return Err(error);
	}

	Ok(())
}

async fn handle_incoming_message_impl(
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

	let thread = if let Some(existing_thread) = existing_thread {
		serenity::ChannelId::new(existing_thread.id)
	} else {
		if is_user_blocked(&data.pg, message.author.id.get()).await? {
			return Ok(());
		}

		let created_at = message.id.created_at();

		let info_builder = serenity::CreateMessage::new()
			.content(make_info_content(
				&data.config,
				message.author.id,
				message.author.id,
				created_at,
				None,
				None,
			))
			.allowed_mentions(data.config.allowed_mentions())
			.embed(make_info_embed(context, &data.config, &message.author).await?);

		let mut forum_post_builder = serenity::CreateForumPost::new(
			format!("Thread from {}", &message.author.tag()),
			info_builder,
		);

		if let Some(open_tag_id) = data.config.forum_channel.open_tag_id {
			forum_post_builder = forum_post_builder.add_applied_tag(open_tag_id)
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

		forum_post.id
	};

	let image_attachment = first_image_attachment(&message.attachments);
	let image_filename = image_attachment.map(|attachment| attachment.filename.clone());
	let cloned_image_attachment = if let Some(attachment) = image_attachment {
		Some(clone_attachment(&context.http, attachment).await?)
	} else {
		None
	};

	let forwarded_message_builder = serenity::CreateMessage::new().add_embed(make_message_embed(
		context,
		&data.config,
		&EmbedOptions {
			user: &message.author,
			content: &message.content,
			image_filename: image_filename.as_deref(),
			outgoing: false,
			anonymous: false,
			user_info: true,
		},
	));

	let fowarded_message = thread
		.send_files(
			context,
			cloned_image_attachment
				.map(|attachment| vec![attachment])
				.unwrap_or_default(),
			forwarded_message_builder,
		)
		.await?;

	insert_received_message(
		&data.pg,
		ReceivedMessage {
			id: message.id.get(),
			thread_id: thread.get(),
			forwarded_message_id: fowarded_message.id.get(),
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
