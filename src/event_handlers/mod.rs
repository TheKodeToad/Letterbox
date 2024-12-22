mod incoming_edit;
mod incoming_message;
mod thread_create_warning;
mod thread_delete;
mod incoming_reaction_add;
mod incoming_reaction_delete;

use incoming_edit::handle_incoming_edit;
use incoming_message::handle_incoming_message;
use incoming_reaction_add::handle_incoming_reaction_add;
use incoming_reaction_delete::handle_incoming_reaction_remove;
use poise::serenity_prelude as serenity;
use thread_create_warning::handle_thread_create_warning;
use thread_delete::handle_thread_delete;

use crate::Data;

pub async fn handle_event(
	context: &serenity::Context,
	event: &serenity::FullEvent,
	data: &Data,
) -> eyre::Result<()> {
	match event {
		serenity::FullEvent::Message { new_message } => {
			handle_incoming_message(context, new_message, data).await?;
			handle_thread_create_warning(context, new_message, data).await?;
		}
		serenity::FullEvent::MessageUpdate { event, .. } => {
			handle_incoming_edit(context, event, data).await?;
		}
		serenity::FullEvent::ReactionAdd { add_reaction } => {
			handle_incoming_reaction_add(context, add_reaction, data).await?;
		}
		serenity::FullEvent::ReactionRemove { removed_reaction } => {
			handle_incoming_reaction_remove(context, removed_reaction, data).await?;
		}
		serenity::FullEvent::ThreadDelete { thread, .. } => {
			handle_thread_delete(thread, data).await?;
		}
		_ => (),
	};

	Ok(())
}
