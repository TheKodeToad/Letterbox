mod incoming_message;
mod outgoing_message;

use incoming_message::handle_incoming_message;
use outgoing_message::handle_outgoing_message;
use poise::serenity_prelude as serenity;

use crate::Data;

pub async fn handle_event(
	context: &serenity::Context,
	event: &serenity::FullEvent,
	data: &Data,
) -> eyre::Result<()> {
	if let serenity::FullEvent::Message { new_message } = event {
		handle_incoming_message(context, new_message, data).await?;
		handle_outgoing_message(context, new_message, data).await?;
	};

	Ok(())
}
