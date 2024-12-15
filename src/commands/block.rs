use poise::serenity_prelude as serenity;

use crate::data::blocked_users::block_user;
use crate::data::blocked_users::is_user_blocked;

use super::util::Context;
use super::util::require_staff;

/// Block a user from creating threads.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff"
)]
pub async fn block(
	context: Context<'_>,
	#[description = "The user to block."] user: serenity::User,
) -> eyre::Result<()> {
	block_impl(context, user, false).await
}

/// Block a user from creating threads without notifying them.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("sblock")
)]
pub async fn silent_block(
	context: Context<'_>,
	#[description = "The user to block."] user: serenity::User,
) -> eyre::Result<()> {
	block_impl(context, user, true).await
}

async fn block_impl(context: Context<'_>, user: serenity::User, silent: bool) -> eyre::Result<()> {
	if user.bot {
		context.reply("❌ Blocking an app has no effect.").await?;
		return Ok(());
	}

	if is_user_blocked(&context.data().pg, user.id.get()).await? {
		context.reply("❌ The specified user is already blocked.").await?;
		return Ok(());
	}

	block_user(&context.data().pg, user.id.get()).await?;

	if !silent {
		user.direct_message(&context.http(), serenity::CreateMessage::new().content("🚫 You have been blocked from creating threads.".to_string())).await.ok();
	}

	if context.author().id == user.id {
		context.reply("✅ Why do this to yourself?").await?;
	} else {
		context.reply(format!("✅ Blocked **{}**!", user.tag().replace("_", "\\_"))).await?;
	}

	Ok(())
}