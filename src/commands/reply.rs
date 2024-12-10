use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateEmbedAuthor;
use poise::serenity_prelude::CreateMessage;

use crate::common::message_as_embed;
use crate::common::message_as_embed_raw;
use crate::data::threads::thread_source_from_target;

use super::common::require_staff;
use super::common::Context;

/// Reply to a ModMail thread.
#[poise::command(slash_command, prefix_command, check = "require_staff")]
pub async fn reply(
	context: Context<'_>,
	#[rest]
	#[description = "The message to send."]
	message: String,
) -> eyre::Result<()> {
	Ok(reply_impl(context, &message, false).await?)
}

/// Reply to a ModMail thread anonymously.
#[poise::command(
	slash_command,
	prefix_command,
	check = "require_staff",
	aliases("anonreply", "anonymousreply")
)]
pub async fn areply(
	context: Context<'_>,
	#[rest]
	#[description = "The message to send."]
	message: String,
) -> eyre::Result<()> {
	Ok(reply_impl(context, &message, true).await?)
}

pub async fn reply_impl(context: Context<'_>, message: &str, anon: bool) -> eyre::Result<()> {
	let Some(dm_channel_id) =
		thread_source_from_target(&context.data().pg, context.channel_id().get()).await?
	else {
		context
			.send(
				poise::CreateReply::default()
					.content("❌ No open thread in this channel.")
					.ephemeral(true),
			)
			.await?;
		return Ok(());
	};
	let dm_channel = serenity::ChannelId::new(dm_channel_id);

	let embed = if let poise::Context::Prefix(prefix) = context {
		message_as_embed(prefix.msg).description(message)
	} else {
		message_as_embed_raw(context.author(), message, &[])
	};

	let mut dm_embed = embed.clone().color(serenity::colours::branding::YELLOW);

	if anon {
		dm_embed = dm_embed.author(CreateEmbedAuthor::new("Staff Team"));
	}

	dm_channel
		.send_message(&context, CreateMessage::new().add_embed(dm_embed))
		.await?;

	context
		.send(
			poise::CreateReply::default()
				.embed(embed.clone().color(serenity::colours::branding::GREEN)),
		)
		.await?;

	if let poise::Context::Prefix(prefix) = context {
		prefix.msg.delete(&context.http()).await?;
	}

	Ok(())
}
