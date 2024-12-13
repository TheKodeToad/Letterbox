use poise::serenity_prelude as serenity;

use crate::data::sent_messages::insert_sent_message;
use crate::data::sent_messages::SentMessage;
use crate::data::threads::get_thread_dm_channel;
use crate::formatting::make_message_embed;
use crate::formatting::EmbedOptions;

use super::common::require_staff;
use super::common::Context;

/// Reply to a ModMail thread.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("r")
)]
pub async fn reply(
	context: Context<'_>,
	#[rest]
	#[description = "The message to send."]
	message: String,
) -> eyre::Result<()> {
	reply_impl(context, &message, false).await
}

/// Reply to a ModMail thread anonymously.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("anonreply", "anonymousreply", "ar")
)]
pub async fn areply(
	context: Context<'_>,
	#[rest]
	#[description = "The message to send."]
	message: String,
) -> eyre::Result<()> {
	reply_impl(context, &message, true).await
}

pub async fn reply_impl(context: Context<'_>, message: &str, anonymous: bool) -> eyre::Result<()> {
	let Some(dm_channel_id) =
		get_thread_dm_channel(&context.data().pg, context.channel_id().get()).await?
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

	let forwarded_message = dm_channel
		.send_message(
			&context,
			serenity::CreateMessage::new().add_embed(make_message_embed(
				context.serenity_context(),
				&context.data().config,
				&EmbedOptions {
					user: context.author(),
					content: message,
					outgoing: false,
					anonymous,
					details: false,
				},
			)),
		)
		.await?;

	let source_message = context
		.send(poise::CreateReply::default().embed(make_message_embed(
			context.serenity_context(),
			&context.data().config,
			&EmbedOptions {
				user: context.author(),
				content: message,
				outgoing: true,
				anonymous,
				details: true,
			},
		)))
		.await?;

	insert_sent_message(
		&context.data().pg,
		SentMessage {
			id: source_message.message().await?.id.get(),
			thread_id: context.channel_id().get(),
			forwarded_message_id: forwarded_message.id.get(),
			author_id: context.author().id.get(),
			anonymous,
		},
	)
	.await?;

	if let poise::Context::Prefix(prefix) = context {
		prefix.msg.delete(&context.http()).await?;
	}

	Ok(())
}
