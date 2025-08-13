use eyre::Result;
use poise::{builtins, samples::HelpConfiguration};

use crate::commands::util::Context;

/// View the help menu.
#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Individual command to show."] command: Option<String>,
) -> Result<()> {
	builtins::help(
		ctx,
		command.as_deref(),
		HelpConfiguration {
			extra_text_at_bottom: "Use /help for more information about a command!",
			..HelpConfiguration::default()
		},
	)
	.await?;

	Ok(())
}
