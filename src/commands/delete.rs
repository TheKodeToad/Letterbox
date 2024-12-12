use eyre::OptionExt;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;

use crate::data::received_messages::get_received_message;
use crate::data::sent_messages::delete_sent_message;
use crate::data::sent_messages::get_sent_message;
use crate::data::threads::get_thread_dm_channel;
use crate::Data;

use super::common::require_staff;
use super::common::Context;

/// Delete a ModMail reply.
#[poise::command(
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("del", "d")
)]
pub async fn delete(context: poise::PrefixContext<'_, Data, eyre::Report>) -> eyre::Result<()> {
	let Some(serenity::MessageReference {
		message_id: Some(message_id),
		..
	}) = context.msg.message_reference
	else {
		context
			.say("❌ Please run this command as a reply to a message.")
			.await?;
		return Ok(());
	};

	let Some(sent_message) = get_sent_message(&context.data.pg, message_id.get()).await? else {
		context
			.say("❌ This message was not sent with the reply command or the thread was closed.")
			.await?;
		return Ok(());
	};

	let dm_channel_id = get_thread_dm_channel(&context.data.pg, sent_message.thread_id)
		.await?
		.ok_or_eyre("Thread went missing!")?;
	let dm_channel = ChannelId::new(dm_channel_id);

	let thread = ChannelId::new(sent_message.thread_id);

	dm_channel
		.delete_message(&context.http(), sent_message.forwarded_message_id)
		.await?;
	thread
		.delete_message(&context.http(), message_id.get())
		.await?;
	delete_sent_message(&context.data.pg, sent_message.id).await?;

	context.msg.delete(&context.http()).await?;

	Ok(())
}
