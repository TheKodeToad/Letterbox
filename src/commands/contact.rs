use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::{
	data::threads::{get_thread_by_user, insert_thread, Thread},
	formatting::{thread_info, user_info_embed},
};

use super::util::{require_staff, Context};

/// Create a new mod-mail thread.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	ephemeral
)]
pub async fn contact(
	context: Context<'_>,
	#[description = "The user to open a thread for."] user: serenity::User,
) -> eyre::Result<()> {
	if user.bot {
		context
			.say("❌ Bot users cannot receive direct messages.")
			.await?;
		return Ok(());
	}

	if let Some(thread) = get_thread_by_user(&context.data().pg, user.id.get()).await? {
		context
			.say(format!(
				"❌ The specified user already has an open thread: {}.",
				serenity::Mention::Channel(serenity::ChannelId::new(thread.id))
			))
			.await?;
		return Ok(());
	}

	context.defer_ephemeral().await?;

	let dm_channel = user.create_dm_channel(context.http()).await?;

	let created_at = context.created_at();

	let info_builder = serenity::CreateMessage::new()
		.content(thread_info::create(
			&context.data().config,
			thread_info::Options {
				user_id: user.id,
				opened: (context.author().id, created_at),
				closed: None,
			},
		))
		.allowed_mentions(thread_info::create_allowed_mentions(&context.data().config))
		.embed(
			user_info_embed::create(context.serenity_context(), &context.data().config, &user)
				.await?,
		);

	let mut forum_post_builder =
		serenity::CreateForumPost::new(format!("Thread for {}", &user.tag()), info_builder);

	if let Some(open_tag_id) = context.data().config.forum_channel.open_tag_id {
		forum_post_builder = forum_post_builder.add_applied_tag(open_tag_id);
	}

	let forum_post = context
		.data()
		.config
		.forum_channel
		.id
		.create_forum_post(&context.http(), forum_post_builder)
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

	context
		.say(format!(
			"✅ Thread opened for {}: {}.",
			user.mention(),
			forum_post.mention()
		))
		.await?;

	Ok(())
}
