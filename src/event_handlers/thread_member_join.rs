use poise::serenity_prelude as serenity;

use crate::{
	data::threads::{self},
	Data,
};

pub async fn handle(
	context: &serenity::Context,
	member: &serenity::Member,
	data: &Data,
) -> eyre::Result<()> {
	if member.guild_id != data.config.server_id {
		return Ok(());
	}

	let Some(thread_data) = threads::get_by_user(&data.pg_pool, member.user.id.get()).await? else {
		return Ok(());
	};

	let thread = serenity::ChannelId::new(thread_data.id);

	thread
		.send_message(
			&context.http,
			serenity::CreateMessage::new().content("📥 The user has rejoined the server."),
		)
		.await?;

	Ok(())
}
