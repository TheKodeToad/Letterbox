use poise::serenity_prelude as serenity;

use crate::data::blocked_users::is_user_blocked;
use crate::data::blocked_users::unblock_user;
use crate::util::markdown::escape_markdown;

use super::util::require_staff;
use super::util::Context;

/// Unblock a user blocked using the block command.
#[poise::command(slash_command, prefix_command, guild_only, check = "require_staff")]
pub async fn unblock(
	context: Context<'_>,
	#[description = "The user to unblock."] user: serenity::User,
) -> eyre::Result<()> {
	unblock_impl(context, user, false).await
}

/// Unblock a user blocked using the block command without notifying them.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("sunblock")
)]
pub async fn silent_unblock(
	context: Context<'_>,
	#[description = "The user to unblock."] user: serenity::User,
) -> eyre::Result<()> {
	unblock_impl(context, user, true).await
}

async fn unblock_impl(
	context: Context<'_>,
	user: serenity::User,
	silent: bool,
) -> eyre::Result<()> {
	if user.bot {
		context.send(poise::CreateReply::default().content("❌ Blocks upon an app have no effect.").ephemeral(true)).await?;
		return Ok(());
	}

	if !is_user_blocked(&context.data().pg, user.id.get()).await? {
		context
			.send(poise::CreateReply::default().content("❌ The specified user is not blocked.").ephemeral(true))
			.await?;
		return Ok(());
	}

	unblock_user(&context.data().pg, user.id.get()).await?;

	if !silent {
		context.defer().await?;

		user.direct_message(
			&context.http(),
			serenity::CreateMessage::new().content("⛓️‍💥 You have been unblocked.".to_string()),
		)
		.await
		.ok();
	}

	context
		.reply(format!(
			"✅ Unblocked **{}**!",
			escape_markdown(&user.tag())
		))
		.await?;

	Ok(())
}
