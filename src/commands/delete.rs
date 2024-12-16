use eyre::OptionExt;
use poise::serenity_prelude as serenity;

use crate::data::sent_messages::delete_sent_message;
use crate::data::sent_messages::get_sent_message;
use crate::data::threads::get_thread;

use super::util::require_staff;
use super::util::Context;
use super::util::PrefixContext;

/// Delete a mod-mail reply.
#[poise::command(
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("del", "d")
)]
pub async fn delete(context: PrefixContext<'_>) -> eyre::Result<()> {
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

	delete_impl(&Context::Prefix(context), message_id).await?;
	context.msg.delete(context.http()).await?;

	Ok(())
}

#[poise::command(context_menu_command = "🗑️ Delete Reply", guild_only, check = "require_staff", ephemeral)]
pub async fn delete_context_menu(context: Context<'_>, message: serenity::Message) -> eyre::Result<()> {
	delete_impl(&context, message.id).await?;
	context.say("✅ Deleted reply.").await?;

	Ok(())
}

pub async fn delete_impl(context: &Context<'_>, message_id: serenity::MessageId) -> eyre::Result<()> {
	let Some(sent_message) = get_sent_message(&context.data().pg, message_id.get()).await? else {
		context
			.say("❌ This message was not sent with the reply command or the thread was closed.")
			.await?;
		return Ok(());
	};

	let dm_channel_id = get_thread(&context.data().pg, sent_message.thread_id)
		.await?
		.ok_or_eyre("Thread went missing!")?
		.dm_channel_id;
	let dm_channel = serenity::ChannelId::new(dm_channel_id);

	let thread = serenity::ChannelId::new(sent_message.thread_id);

	context.defer_ephemeral().await?;

	dm_channel
		.delete_message(&context.http(), sent_message.forwarded_message_id)
		.await?;
	thread
		.delete_message(&context.http(), message_id.get())
		.await?;
	delete_sent_message(&context.data().pg, sent_message.id).await?;

	Ok(())
}