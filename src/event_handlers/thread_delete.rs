use poise::serenity_prelude as serenity;

use crate::{
	data::threads::{self},
	Data,
};

pub async fn handle(thread: &serenity::PartialGuildChannel, data: &Data) -> eyre::Result<()> {
	if threads::get(&data.pg, thread.id.get()).await?.is_none() {
		return Ok(());
	}

	threads::delete(&data.pg, thread.id.get()).await?;

	Ok(())
}
