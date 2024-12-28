use poise::serenity_prelude as serenity;

use crate::{data::sent_messages::get_sent_message_by_forwarded_message, Data};

pub async fn handle_incoming_reaction_remove(context: &serenity::Context, reaction: &serenity::Reaction, data: &Data) -> eyre::Result<()> {
	let Some(sent_message) = get_sent_message_by_forwarded_message(&data.pg, reaction.message_id.get()).await? else {
		return Ok(());
	};

	let thread = serenity::ChannelId::new(sent_message.thread_id);
	let source_message = serenity::MessageId::new(sent_message.id);

	let current_user_id = context.cache.current_user().id;
	// the reaction might have been added whilst offline, it's okay to ignore
	thread.delete_reaction(&context.http, source_message, Some(current_user_id), reaction.emoji.clone()).await.ok();

	Ok(())
}