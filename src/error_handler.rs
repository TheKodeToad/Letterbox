use crate::Data;

const GENERIC_ERROR: &str = "❌ Something went wrong whilst executing this command.";

pub async fn handle_error(error: poise::FrameworkError<'_, Data, eyre::Report>) {
	match error {
		poise::FrameworkError::Command {
			error,
			ctx: context,
			..
		} => {
			log::error!(
				"Error executing command {}:\n{error:#?}",
				context.command().name
			);

			context.say(GENERIC_ERROR).await.ok();
		}
		poise::FrameworkError::CommandCheckFailed {
			error,
			ctx: context,
			..
		} => {
			if let Some(error) = error {
				log::error!(
					"Error checking permissions for {}:\n{error:#?}",
					context.command().name
				);

				context.say(GENERIC_ERROR).await.ok();
			} else if matches!(context, poise::Context::Application(_)) {
				context
					.send(
						poise::CreateReply::default()
							.content("❌ Permission denied.")
							.ephemeral(true),
					)
					.await
					.ok();
			}
		}
		_ => {
			if let Err(error) = poise::builtins::on_error(error).await {
				log::error!("Unhandled error in Poise's built in error handler:{error:#?}");
			}
		}
	}
}
