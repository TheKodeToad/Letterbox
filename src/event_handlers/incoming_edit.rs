use poise::serenity_prelude as serenity;

use crate::{data::received_messages::get_received_message, formatting::make_embed, Data};

pub async fn handle_incoming_edit(
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
	let Some(ref content) = message.content else {
		// content not changed, only thing we really care about
		return Ok(());
	};

	let Some(received_message) = get_received_message(&data.pg, message.id.get()).await? else {
		return Ok(());
	};

	let thread = serenity::ChannelId::new(received_message.thread_id);

	thread
		.edit_message(
			&context.http,
			received_message.forwarded_message_id,
			serenity::EditMessage::new().add_embed(make_embed(
				context,
				&data.config,
				author,
				content,
				false,
				false,
				true,
			)),
		)
		.await?;

	Ok(())
}
