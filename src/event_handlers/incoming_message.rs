use poise::serenity_prelude as serenity;

use crate::{
	common::message_as_embed,
	data::threads::{link_thread_source_to_target, thread_target_from_source},
	Data,
};

pub async fn handle_incoming_message(
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

	let existing_thread_id = thread_target_from_source(&data.pg, message.channel_id.get()).await?;

	if let Some(existing_thread_id) = existing_thread_id {
		let existing_thread = serenity::ChannelId::new(existing_thread_id);
		existing_thread
			.send_message(
				context,
				serenity::CreateMessage::new().add_embed(
					message_as_embed(message).color(serenity::colours::branding::YELLOW),
				),
			)
			.await?;
	} else {
		let forum_post = data
			.config
			.forum_channel_id
			.create_forum_post(
				context,
				serenity::CreateForumPost::new(
					"Thread from ".to_string() + &message.author.tag(),
					serenity::CreateMessage::new().add_embed(
						message_as_embed(message).color(serenity::colours::branding::YELLOW),
					),
				),
			)
			.await?;

		link_thread_source_to_target(&data.pg, message.channel_id.get(), forum_post.id.get())
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
