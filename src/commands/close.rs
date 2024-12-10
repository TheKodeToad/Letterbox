use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateAllowedMentions;
use poise::serenity_prelude::CreateMessage;

use super::common::require_moderator;
use super::common::Context;
use crate::data::threads::delete_thread_by_source;
use crate::data::threads::thread_source_from_target;

#[poise::command(slash_command, check = "require_moderator")]
pub async fn close(context: Context<'_>) -> eyre::Result<()> {
	let Some(dm_channel_id) =
		thread_source_from_target(&context.data().pg, context.channel_id().get()).await?
	else {
		context
			.send(
				poise::CreateReply::default()
					.content("❌ No open thread in this channel")
					.ephemeral(true),
			)
			.await?;
		return Ok(());
	};
	let dm_channel = serenity::ChannelId::new(dm_channel_id);

	context.defer().await?;

	delete_thread_by_source(&context.data().pg, dm_channel_id).await?;

	let close_message = format!("⛔ Thread closed by <@{}>.", context.author().id);

	dm_channel
		.send_message(
			&context.http(),
			CreateMessage::new()
				.content(&close_message)
				.allowed_mentions(CreateAllowedMentions::new()),
		)
		.await?;
	context.say(&close_message).await?;

	Ok(())
}
