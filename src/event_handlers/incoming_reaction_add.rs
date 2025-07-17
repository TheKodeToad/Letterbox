use poise::serenity_prelude as serenity;

use crate::{data::sent_messages, Data};

pub async fn handle(
	context: &serenity::Context,
	reaction: &serenity::Reaction,
	data: &Data,
) -> eyre::Result<()> {
	let Some(sent_message) =
		sent_messages::get_by_forwarded(&data.pg, reaction.message_id.get()).await?
	else {
		return Ok(());
	};

	let thread = serenity::ChannelId::new(sent_message.thread_id);
	let source_message = serenity::MessageId::new(sent_message.id);

	// the bot might not be in the server with the correct emoji, so ignore errors
	thread
		.create_reaction(&context.http, source_message, reaction.emoji.clone())
		.await
		.ok();

	Ok(())
}
