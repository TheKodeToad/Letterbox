use poise::serenity_prelude as serenity;

use crate::{data::sent_messages::get_sent_message_by_forwarded_message, Data};

pub async fn handle_incoming_reaction_add(context: &serenity::Context, reaction: &serenity::Reaction, data: &Data) -> eyre::Result<()> {
	let Some(sent_message) = get_sent_message_by_forwarded_message(&data.pg, reaction.message_id.get()).await? else {
		return Ok(());
	};

	let thread = serenity::ChannelId::new(sent_message.thread_id);
	let source_message = serenity::MessageId::new(sent_message.id);

	// the bot might not be in the server with the correct emoji, so ignore errors
	thread.create_reaction(&context.http, source_message, reaction.emoji.clone()).await.ok();

	Ok(())
}