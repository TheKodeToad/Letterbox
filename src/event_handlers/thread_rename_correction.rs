use poise::serenity_prelude as serenity;

use crate::{data::threads::get_thread, formatting::make_thread_title, Data};

pub async fn handle_thread_rename_correction(
	context: &serenity::Context,
	thread: &serenity::GuildChannel,
	data: &Data,
) -> eyre::Result<()> {
	if get_thread(&data.pg, thread.id.get()).await?.is_none() {
		log::debug!(
			"Not handling thread update for {} as it is not in db",
			thread.id
		);
		return Ok(());
	}

	let correct_title = make_thread_title(&thread.name, true);

	if thread.name != correct_title {
		log::debug!(
			"Changing thread name of {} from {} to {}",
			thread.id,
			thread.name,
			correct_title
		);
		thread
			.id
			.edit_thread(&context, serenity::EditThread::new().name(correct_title))
			.await?;
	}

	Ok(())
}
