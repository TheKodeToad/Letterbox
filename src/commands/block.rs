use poise::serenity_prelude as serenity;

use crate::data::blocked_users;
use crate::util::markdown;

use super::util::require_staff;
use super::util::Context;

/// Block a user from creating threads.
#[poise::command(slash_command, prefix_command, guild_only, check = "require_staff")]
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
		context
			.send(
				poise::CreateReply::default()
					.content("❌ Blocks upon an app have no effect.")
					.ephemeral(true),
			)
			.await?;
		return Ok(());
	}

	let blocked = blocked_users::add(&context.data().pg, user.id.get()).await?;

	if !blocked {
		context
			.send(
				poise::CreateReply::default()
					.content("❌ The specified user is already blocked.")
					.ephemeral(true),
			)
			.await?;
		return Ok(());
	}

	if !silent {
		context.defer().await?;

		user.direct_message(
			&context.http(),
			serenity::CreateMessage::new()
				.content("🚫 You have been blocked from creating threads.".to_string()),
		)
		.await
		.ok();
	}

	if context.author().id == user.id {
		context.reply("✅ Why do this to yourself?").await?;
	} else {
		context
			.reply(format!("✅ Blocked **{}**!", markdown::escape(&user.tag())))
			.await?;
	}

	Ok(())
}
