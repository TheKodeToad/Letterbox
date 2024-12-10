use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateAllowedMentions;
use poise::serenity_prelude::CreateMessage;
use poise::serenity_prelude::EditThread;

use super::common::require_staff;
use super::common::Context;
use crate::data::threads::delete_thread_by_source;
use crate::data::threads::thread_source_from_target;

/// Close a ModMail thread.
#[poise::command(slash_command, prefix_command, check = "require_staff")]
pub async fn close(context: Context<'_>) -> eyre::Result<()> {
	Ok(close_impl(context, false).await?)
}

/// Close a ModMail thread anonymously.
#[poise::command(
	slash_command,
	prefix_command,
	check = "require_staff",
	aliases("anonclose", "anonymousclose")
)]
pub async fn aclose(context: Context<'_>) -> eyre::Result<()> {
	Ok(close_impl(context, true).await?)
}

async fn close_impl(context: Context<'_>, anon: bool) -> eyre::Result<()> {
	let Some(dm_channel_id) =
		thread_source_from_target(&context.data().pg, context.channel_id().get()).await?
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
	let dm_channel = serenity::ChannelId::new(dm_channel_id);

	context.defer().await?;

	delete_thread_by_source(&context.data().pg, dm_channel_id).await?;

	let close_message = if anon {
		format!("⛔ Thread closed.")
	} else {
		format!("⛔ Thread closed by <@{}>.", context.author().id)
	};

	dm_channel
		.send_message(
			&context.http(),
			CreateMessage::new()
				.content(&close_message)
				.allowed_mentions(CreateAllowedMentions::new()),
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
			EditThread::new().locked(true).archived(true),
		)
		.await?;

	Ok(())
}
