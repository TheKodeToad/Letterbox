use poise::serenity_prelude as serenity;

use crate::{
	data::threads::{delete_thread, get_thread},
	Data,
};

pub async fn handle_thread_delete(
	thread: &serenity::PartialGuildChannel,
	data: &Data,
) -> eyre::Result<()> {
	if get_thread(&data.pg, thread.id.get()).await?.is_none() {
		return Ok(());
	}

	delete_thread(&data.pg, thread.id.get()).await?;

	Ok(())
}
