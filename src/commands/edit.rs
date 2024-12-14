use eyre::OptionExt;
use poise::serenity_prelude as serenity;

use crate::{
	data::{
		sent_messages::{get_sent_message},
		threads::get_thread,
	},
	formatting::{make_message_embed, EmbedOptions},
};

use super::common::require_staff;
use super::common::PrefixContext;

/// Edit a ModMail reply.
#[poise::command(
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("e", "edit")
)]
pub async fn edit(
	context: PrefixContext<'_>,
	#[rest]
	#[description = "The new message content"]
	content: String,
) -> eyre::Result<()> {
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

	if sent_message.author_id != context.author().id.get() {
		context
			.say("❌ This reply was not authored by you.")
			.await?;
		return Ok(());
	}

	let dm_channel_id = get_thread(&context.data.pg, sent_message.thread_id)
		.await?
		.ok_or_eyre("Thread went missing!")?
		.dm_channel_id;
	let dm_channel = serenity::ChannelId::new(dm_channel_id);

	let thread = serenity::ChannelId::new(sent_message.thread_id);

	dm_channel
		.edit_message(
			&context.http(),
			sent_message.forwarded_message_id,
			serenity::EditMessage::new().embed(make_message_embed(
				context.serenity_context,
				&context.data().config,
				&EmbedOptions {
					user: context.author(),
					content: &content,
					outgoing: false,
					anonymous: sent_message.anonymous,
					details: false,
				},
			)),
		)
		.await?;
	thread
		.edit_message(
			&context.http(),
			message_id,
			serenity::EditMessage::new().embed(make_message_embed(
				context.serenity_context,
				&context.data().config,
				&EmbedOptions {
					user: context.author(),
					content: &content,
					outgoing: true,
					anonymous: sent_message.anonymous,
					details: true,
				},
			)),
		)
		.await?;

	context.msg.delete(&context.http()).await?;

	Ok(())
}
