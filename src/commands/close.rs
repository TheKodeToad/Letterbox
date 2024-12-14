use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;

use super::common::require_staff;
use super::common::Context;
use crate::data::threads::delete_thread;
use crate::data::threads::get_thread;

/// Close a mod-mail thread.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("c")
)]
pub async fn close(context: Context<'_>) -> eyre::Result<()> {
	close_impl(context, false).await
}

/// Close a mod-mail thread anonymously.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("ac", "anonclose")
)]
pub async fn aclose(context: Context<'_>) -> eyre::Result<()> {
	close_impl(context, true).await
}

async fn close_impl(context: Context<'_>, anonymous: bool) -> eyre::Result<()> {
	let Some(thread) = get_thread(&context.data().pg, context.channel_id().get()).await? else {
		context
			.send(
				poise::CreateReply::default()
					.content("❌ No open thread in this channel.")
					.ephemeral(true),
			)
			.await?;
		return Ok(());
	};
	let dm_channel = serenity::ChannelId::new(thread.dm_channel_id);

	context.defer().await?;

	delete_thread(&context.data().pg, context.channel_id().get()).await?;

	let close_message = if anonymous {
		"⛔ Thread closed.".to_string()
	} else {
		format!("⛔ Thread closed by {}.", context.author().mention())
	};

	dm_channel
		.send_message(
			&context.http(),
			serenity::CreateMessage::new()
				.content(&close_message)
				.allowed_mentions(serenity::CreateAllowedMentions::new()),
		)
		.await?;
	context.say(&close_message).await?;

	if let Context::Prefix(prefix) = context {
		prefix.msg.delete(context).await.ok();
	}

	context
		.channel_id()
		.edit_thread(
			&context.http(),
			serenity::EditThread::new().locked(true).archived(true),
		)
		.await?;

	Ok(())
}
