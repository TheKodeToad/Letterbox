use std::time::Duration;

use poise::serenity_prelude as serenity;

use crate::commands::util::{require_staff, Context};

/// View information about the app.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	ephemeral,
	check = "require_staff"
)]
pub async fn info(context: Context<'_>) -> eyre::Result<()> {
	let version = env!("CARGO_PKG_VERSION");
	let latency = context.ping().await;

	// we could also retrieve the name from cargo.toml but it's not formatted nicely :(
	let mut embed = serenity::CreateEmbed::new()
		.color(serenity::colours::branding::BLURPLE)
		.title("Letterbox")
		.description("A lightweight mod-mail app for Discord!")
		.field("Version", version, false);

	if latency != Duration::ZERO {
		embed = embed.field("Latency", format!("{}ms", latency.as_millis()), false)
	}

	context
		.send(poise::CreateReply::default().embed(embed))
		.await?;

	Ok(())
}
