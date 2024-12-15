use poise::serenity_prelude as serenity;

use crate::Data;

pub async fn handle_thread_create_warning(
	context: &serenity::Context,
	message: &serenity::Message,
	data: &Data,
) -> eyre::Result<()> {
	// Threads starter messages have the same ID as the thread
	if message.id.get() != message.channel_id.get() {
		return Ok(());
	}

	if message.author.id == context.cache.current_user().id {
		return Ok(());
	}

	let serenity::Channel::Guild(channel) = message.channel(&context.http).await? else {
		return Ok(());
	};
	let Some(parent_id) = channel.parent_id else {
		return Ok(());
	};

	if parent_id != data.config.forum_channel.id {
		return Ok(());
	}

	message
		.reply(
			&context.http,
			"⚠️ The **contact** command must be used if you wish to open a new thread.",
		)
		.await?;

	Ok(())
}
