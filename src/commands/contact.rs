use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::{
	data::threads::{get_thread_by_user, insert_thread, Thread},
	formatting::{make_info_content, make_info_embed},
};

use super::common::{require_staff, Context};

/// Create a new mod-mail thread.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("c")
)]
pub async fn contact(
	context: Context<'_>,
	#[description = "The user to open a thread for."] user: serenity::User,
) -> eyre::Result<()> {
	if user.bot {
		context
			.send(poise::CreateReply::default().content("❌ Bot users cannot receive direct messages.").ephemeral(true))
			.await?;
		return Ok(());
	}

	if let Some(thread) = get_thread_by_user(&context.data().pg, user.id.get()).await? {
		context
			.send(poise::CreateReply::default().content(format!(
				"❌ The specified user already has an open thread: {}.",
				serenity::Mention::Channel(serenity::ChannelId::new(thread.id))
			)).ephemeral(true))
			.await?;
		return Ok(());
	}

	context.defer().await?;

	let dm_channel = user.create_dm_channel(context.http()).await?;

	let created_at = context.created_at();

	let forum_post = context
		.data()
		.config
		.forum_channel_id
		.create_forum_post(
			&context.http(),
			serenity::CreateForumPost::new(
				format!("Thread for {}", &user.tag()),
				serenity::CreateMessage::new()
					.content(make_info_content(
						&context.data().config,
						user.id,
						context.author().id,
						created_at,
					))
					.allowed_mentions(context.data().config.allowed_mentions())
					.embed(
						make_info_embed(context.serenity_context(), &context.data().config, &user)
							.await?,
					),
			),
		)
		.await?;

	insert_thread(
		&context.data().pg,
		Thread {
			id: forum_post.id.get(),
			dm_channel_id: dm_channel.id.get(),
			user_id: user.id.get(),
			opened_by_id: context.author().id.get(),
			created_at: *created_at,
		},
	)
	.await?;

	context.say(format!("✅ Thread opened for {}: {}", user.mention(), forum_post.mention())).await?;

	Ok(())
}
