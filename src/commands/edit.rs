use eyre::OptionExt;
use poise::{
	serenity_prelude::{self as serenity},
	Modal,
};

use crate::{
	data::{sent_messages, threads},
	formatting::message_embed,
};

use super::util::PrefixContext;
use super::util::{require_staff, ApplicationContext, Context};

#[derive(poise::Modal)]
#[name = "Edit Message"]
struct EditDialog {
	#[name = "Content"]
	#[placeholder = "The new message content"]
	content: String,
}

/// Edit a mod-mail reply.
#[poise::command(
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("e", "edit")
)]
pub async fn edit(
	ctx: PrefixContext<'_>,
	#[rest]
	#[description = "The new message content"]
	content: String,
) -> eyre::Result<()> {
	let Some(serenity::MessageReference {
		message_id: Some(message_id),
		..
	}) = ctx.msg.message_reference
	else {
		ctx.say("❌ Please run this command as a reply to a message.")
			.await?;
		return Ok(());
	};

	if create(Context::Prefix(ctx), message_id, content).await? {
		ctx.msg.delete(ctx.http()).await?;
	}

	Ok(())
}

#[poise::command(
	context_menu_command = "✏ Edit Reply",
	guild_only,
	check = "require_staff",
	ephemeral
)]
pub async fn edit_context_menu(
	context: ApplicationContext<'_>,
	message: serenity::Message,
) -> eyre::Result<()> {
	let Some(fields) = EditDialog::execute(context).await? else {
		return Ok(());
	};

	create(Context::Application(context), message.id, fields.content).await?;

	Ok(())
}

async fn create(
	ctx: Context<'_>,
	message_id: serenity::MessageId,
	content: String,
) -> eyre::Result<bool> {
	let Some(sent_message) = sent_messages::get(&ctx.data().pg, message_id.get()).await? else {
		ctx.say("❌ This message was not sent with the reply command or the thread was closed.")
			.await?;
		return Ok(false);
	};

	if sent_message.author_id != ctx.author().id.get() {
		ctx.say("❌ This reply was not authored by you.").await?;
		return Ok(false);
	}

	let dm_channel_id = threads::get(&ctx.data().pg, sent_message.thread_id)
		.await?
		.ok_or_eyre("Thread went missing!")?
		.dm_channel_id;
	let dm_channel = serenity::ChannelId::new(dm_channel_id);

	let thread = serenity::ChannelId::new(sent_message.thread_id);

	let forwarded_message_builder = serenity::EditMessage::new().embed(message_embed::create(
		ctx.serenity_context(),
		&ctx.data().config,
		message_embed::Options {
			author: ctx.author(),
			content: &content,
			image_filename: sent_message.image_filename.as_deref(),
			outgoing: false,
			anonymous: sent_message.anonymous,
			user_info: false,
		},
	));

	dm_channel
		.edit_message(
			&ctx.http(),
			sent_message.forwarded_message_id,
			forwarded_message_builder,
		)
		.await?;

	let source_message_builder = serenity::EditMessage::new().embed(message_embed::create(
		ctx.serenity_context(),
		&ctx.data().config,
		message_embed::Options {
			author: ctx.author(),
			content: &content,
			image_filename: sent_message.image_filename.as_deref(),
			outgoing: true,
			anonymous: sent_message.anonymous,
			user_info: true,
		},
	));

	thread
		.edit_message(&ctx.http(), message_id, source_message_builder)
		.await?;

	Ok(true)
}
