mod incoming_edit;
mod incoming_message;
mod incoming_reaction_add;
mod incoming_reaction_delete;
mod thread_create_warning;
mod thread_delete;
mod thread_member_join;
mod thread_member_leave;

use poise::serenity_prelude as serenity;

use crate::Data;

pub async fn handle_event(
	context: &serenity::Context,
	event: &serenity::FullEvent,
	data: &Data,
) -> eyre::Result<()> {
	match event {
		serenity::FullEvent::Message { new_message } => {
			incoming_message::handle(context, new_message, data).await?;
			thread_create_warning::handle(context, new_message, data).await?;
		}
		serenity::FullEvent::MessageUpdate { event, .. } => {
			incoming_edit::handle(context, event, data).await?;
		}
		serenity::FullEvent::ReactionAdd { add_reaction } => {
			incoming_reaction_add::handle(context, add_reaction, data).await?;
		}
		serenity::FullEvent::ReactionRemove { removed_reaction } => {
			incoming_reaction_delete::handle(context, removed_reaction, data).await?;
		}
		serenity::FullEvent::ThreadDelete { thread, .. } => {
			thread_delete::handle(thread, data).await?;
		}
		serenity::FullEvent::GuildMemberAddition { new_member, .. } => {
			thread_member_join::handle(context, new_member, data).await?;
		}
		serenity::FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
			thread_member_leave::handle(context, *guild_id, user, data).await?;
		}
		_ => (),
	};

	Ok(())
}
