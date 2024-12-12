mod incoming_edit;
mod incoming_message;

use incoming_edit::handle_incoming_edit;
use incoming_message::handle_incoming_message;
use poise::serenity_prelude as serenity;

use crate::Data;

pub async fn handle_event(
	context: &serenity::Context,
	event: &serenity::FullEvent,
	data: &Data,
) -> eyre::Result<()> {
	match event {
		serenity::FullEvent::Message { new_message } => {
			handle_incoming_message(context, new_message, data).await?
		}
		serenity::FullEvent::MessageUpdate { event, .. } => {
			handle_incoming_edit(context, event, data).await?
		}
		_ => (),
	};

	Ok(())
}
