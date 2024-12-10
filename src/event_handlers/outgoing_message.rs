use poise::serenity_prelude::{self as serenity, MessageType};

use crate::{common::message_as_embed, data::threads::thread_source_from_target, Data};

// just reimplementing prefix commands for now
// because I don't think poise's commands support unquoted strings lol
pub async fn handle_outgoing_message(
	context: &serenity::Context,
	message: &serenity::Message,
	data: &Data,
) -> eyre::Result<()> {
	if message.author.bot {
		return Ok(());
	}

	if !(message.kind == MessageType::Regular || message.kind == MessageType::InlineReply) {
		return Ok(());
	}

	let Some(ref member) = message.member else {
		return Ok(());
	};

	if !data.config.is_moderator(&member.roles) {
		return Ok(());
	}

	let Some(chopped) = message.content.strip_prefix(&data.config.prefix) else {
		return Ok(());
	};
	let chopped = chopped.trim_start();

	let Some(content) = chopped.strip_prefix("reply ") else {
		return Ok(());
	};

	let Some(dm_channel_id) = thread_source_from_target(&data.pg, message.channel_id.get()).await?
	else {
		return Ok(());
	};
	let dm_channel = serenity::ChannelId::new(dm_channel_id);

	let embed = message_as_embed(message);
	dm_channel
		.send_message(
			&context,
			serenity::CreateMessage::new().add_embed(
				embed
					.clone()
					.color(serenity::colours::branding::YELLOW)
					.description(content),
			),
		)
		.await?;

	message
		.channel_id
		.send_message(
			&context,
			serenity::CreateMessage::new().add_embed(
				embed
					.clone()
					.color(serenity::colours::branding::GREEN)
					.description(content),
			),
		)
		.await?;
	message.delete(&context.http).await?;

	Ok(())
}
