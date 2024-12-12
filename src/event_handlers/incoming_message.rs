use poise::serenity_prelude as serenity;

use crate::{
	data::{
		received_messages::{insert_received_message, ReceivedMessage},
		threads::{get_thread_id, insert_thread},
	},
	formatting::message_as_embed,
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

	let existing_thread_id = get_thread_id(&data.pg, message.channel_id.get()).await?;

	if let Some(existing_thread_id) = existing_thread_id {
		let existing_thread = serenity::ChannelId::new(existing_thread_id);
		let fowarded_message = existing_thread
			.send_message(
				context,
				serenity::CreateMessage::new().add_embed(
					message_as_embed(message).color(serenity::colours::branding::YELLOW),
				),
			)
			.await?;

		insert_received_message(
			&data.pg,
			ReceivedMessage {
				id: message.id.get(),
				thread_id: existing_thread_id,
				forwarded_message_id: fowarded_message.id.get(),
			},
		)
		.await?;
	} else {
		let forum_post = data
			.config
			.forum_channel_id
			.create_forum_post(
				&context.http,
				serenity::CreateForumPost::new(
					"Thread from ".to_string() + &message.author.tag(),
					serenity::CreateMessage::new().add_embed(
						message_as_embed(message).color(serenity::colours::branding::YELLOW),
					),
				),
			)
			.await?;

		insert_thread(&data.pg, forum_post.id.get(), message.channel_id.get()).await?;

		insert_received_message(
			&data.pg,
			ReceivedMessage {
				id: message.id.get(),
				thread_id: forum_post.id.get(),
				forwarded_message_id: forum_post.id.get(), // first message in thread always has thread id
			},
		)
		.await?;
	}

	message
		.react(
			&context.http,
			serenity::ReactionType::Unicode("✅".to_string()),
		)
		.await?;

	Ok(())
}
