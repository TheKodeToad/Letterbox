use eyre::eyre;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;

use super::util::require_staff;
use super::util::Context;
use crate::data::threads::delete_thread;
use crate::data::threads::get_thread;
use crate::formatting::thread_info;
use crate::util::markdown;

/// Close a mod-mail thread.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("c")
)]
pub async fn close(context: Context<'_>) -> eyre::Result<()> {
	close_impl(context, false, false).await
}

/// Close a mod-mail thread anonymously.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("ac", "aclose")
)]
pub async fn anon_close(context: Context<'_>) -> eyre::Result<()> {
	close_impl(context, false, true).await
}

/// Close a mod-mail thread without sending the "Thread closed" message.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("sc", "sclose")
)]
pub async fn silent_close(context: Context<'_>) -> eyre::Result<()> {
	close_impl(context, true, true).await
}

async fn close_impl(context: Context<'_>, silent: bool, anonymous: bool) -> eyre::Result<()> {
	let Some(thread_data) = get_thread(&context.data().pg, context.channel_id().get()).await?
	else {
		context
			.send(
				poise::CreateReply::default()
					.content("❌ No open thread in this channel.")
					.ephemeral(true),
			)
			.await?;
		return Ok(());
	};

	let dm_channel = serenity::ChannelId::new(thread_data.dm_channel_id);

	context.defer().await?;

	delete_thread(&context.data().pg, context.channel_id().get()).await?;

	let mut dm_channel_notification = if anonymous {
		"⛔ Thread closed.".to_string()
	} else {
		format!(
			"⛔ Thread closed by **{}**.",
			markdown::escape(context.author().display_name())
		)
	};
	if let Some(suffix) = &context.data().config.messages.thread_closed {
		dm_channel_notification.push('\n');
		dm_channel_notification += suffix;
	}
	if !silent {
		dm_channel
			.send_message(
				&context.http(),
				serenity::CreateMessage::new()
					.content(dm_channel_notification)
					.allowed_mentions(serenity::CreateAllowedMentions::new()),
			)
			.await
			.ok();
	}

	context
		.say(format!(
			"⛔ Thread closed by **{}**.",
			&context.author().mention()
		))
		.await?;

	if let Context::Prefix(prefix) = context {
		prefix.msg.delete(context).await.ok();
	}

	let serenity::Channel::Guild(mut thread) =
		context.channel_id().to_channel(&context.http()).await?
	else {
		return Err(eyre!("Channel is not guild channel!"));
	};

	let info_builder = serenity::EditMessage::new()
		.content(thread_info::create(
			&context.data().config,
			thread_info::Options {
				user_id: serenity::UserId::new(thread_data.user_id),
				opened: (
					serenity::UserId::new(thread_data.opened_by_id),
					thread_data.created_at.into(),
				),
				closed: Some((context.author().id, context.created_at())),
			},
		))
		.allowed_mentions(thread_info::create_allowed_mentions(&context.data().config));

	thread
		.edit_message(&context.http(), thread.id.get(), info_builder)
		.await?;

	let mut edit_thread_builder = serenity::EditThread::new().locked(true).archived(true);

	if let Some(closed_tag_id) = context.data().config.forum_channel.closed_tag_id {
		edit_thread_builder = edit_thread_builder.applied_tags([closed_tag_id]);
	}

	thread
		.edit_thread(&context.http(), edit_thread_builder)
		.await?;

	Ok(())
}
