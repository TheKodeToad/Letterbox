use poise::serenity_prelude as serenity;

use crate::{data::threads::get_thread_by_user, Data};

pub async fn handle(
	context: &serenity::Context,
	guild_id: serenity::GuildId,
	user: &serenity::User,
	data: &Data,
) -> eyre::Result<()> {
	if guild_id != data.config.server_id {
		return Ok(());
	}

	let Some(thread_data) = get_thread_by_user(&data.pg, user.id.get()).await? else {
		return Ok(());
	};

	let thread = serenity::ChannelId::new(thread_data.id);

	thread
		.send_message(
			&context.http,
			serenity::CreateMessage::new().content("📤 The user has left the server."),
		)
		.await?;

	Ok(())
}
