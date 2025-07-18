use poise::serenity_prelude as serenity;

use crate::{
	data::received_messages::{self},
	formatting::message_embed,
	Data,
};

pub async fn handle(
	context: &serenity::Context,
	message: &serenity::MessageUpdateEvent,
	data: &Data,
) -> eyre::Result<()> {
	if message.guild_id.is_some() {
		return Ok(());
	}

	// TODO: why would author be missing
	// attachments also will be lost if just the content changes and vice versa
	// perhaps let's just store this stuff in the database?
	// it'll all be gone when the thread is closed so it's fine
	let Some(ref author) = message.author else {
		return Ok(());
	};
	let Some(ref message_content) = message.content else {
		// content not changed, only thing we really care about
		return Ok(());
	};

	let Some(received_message) = received_messages::get(&data.pg, message.id.get()).await? else {
		return Ok(());
	};

	let reaction = serenity::ReactionType::Unicode("⌛".to_string());

	message
		.channel_id
		.create_reaction(&context.http, message.id, reaction.clone())
		.await?;

	let thread = serenity::ChannelId::new(received_message.thread_id);
	let forwarded_message_builder = serenity::EditMessage::new().add_embed(message_embed::create(
		context,
		&data.config,
		message_embed::Options {
			author,
			content: message_content,
			image_filename: received_message.image_filename.as_deref(),
			outgoing: false,
			anonymous: false,
			user_info: true,
		},
	));

	let edit_result = thread
		.edit_message(
			&context.http,
			received_message.forwarded_message_id,
			forwarded_message_builder,
		)
		.await;

	if edit_result.is_err() {
		message
			.channel_id
			.send_message(
				&context.http,
				serenity::CreateMessage::new()
					.content("❌ An error occurred - your edit did not go through.")
					.reference_message(
						serenity::MessageReference::new(
							serenity::MessageReferenceKind::Default,
							message.channel_id,
						)
						.message_id(message.id),
					)
					.allowed_mentions(serenity::CreateAllowedMentions::new()),
			)
			.await?;
	}

	let bot_user_id = context.cache.current_user().id;

	message
		.channel_id
		.delete_reaction(
			&context.http,
			message.id,
			Some(bot_user_id),
			reaction.clone(),
		)
		.await?;

	Ok(())
}
