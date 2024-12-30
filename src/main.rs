use std::path::Path;
use std::sync::Arc;

use commands::commands;
use config::Config;
use data::migrations;
use error_handler::handle_error;
use event_handlers::handle_event;
use log::warn;
use poise::serenity_prelude as serenity;
use tokio::signal::ctrl_c;
#[cfg(target_family = "unix")]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(target_family = "windows")]
use tokio::signal::windows::ctrl_close;

mod commands;
mod config;
mod data;
mod error_handler;
mod event_handlers;
mod formatting;
mod util;

pub struct Data {
	config: Config,
	pg: tokio_postgres::Client,
}

enum ShutdownReason {
	Sigterm,
	CtrlC,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
	dotenvy::dotenv().ok();
	color_eyre::install()?;
	env_logger::init();

	let bot_token = std::env::var("DISCORD_BOT_TOKEN")?;

	let options = poise::FrameworkOptions {
		commands: commands(),
		allowed_mentions: Some(serenity::CreateAllowedMentions::new()),
		prefix_options: poise::PrefixFrameworkOptions {
			dynamic_prefix: Some(|context| {
				Box::pin(async move { Ok(Some(context.data.config.prefix.clone())) })
			}),
			..Default::default()
		},
		event_handler: |context, event, _framework, data| {
			Box::pin(handle_event(context, event, data))
		},
		on_error: |error| Box::pin(handle_error(error)),
		..Default::default()
	};
	let framework = poise::Framework::builder()
		.options(options)
		.setup(|context, _ready, framework| Box::pin(setup(context, framework)))
		.build();

	let intents = serenity::GatewayIntents::non_privileged()
		| serenity::GatewayIntents::MESSAGE_CONTENT
		| serenity::GatewayIntents::GUILD_MEMBERS;

	let mut client = serenity::ClientBuilder::new(bot_token, intents)
		.framework(framework)
		.await?;

	let shard_manager = client.shard_manager.clone();

	#[cfg(target_family = "unix")]
	let mut sigterm = signal(SignalKind::terminate())?;
	#[cfg(target_family = "windows")]
	let mut sigterm = ctrl_close()?;

	tokio::select! {
		result = client.start() => result.map_err(eyre::Report::from),
		_ = sigterm.recv() => {
			handle_shutdown(shard_manager, ShutdownReason::Sigterm).await;
			std::process::exit(0);
		}
		_ = ctrl_c() => {
			handle_shutdown(shard_manager, ShutdownReason::CtrlC).await;
			std::process::exit(130);
		}
	}
}

async fn setup(
	context: &serenity::Context,
	framework: &poise::Framework<Data, eyre::Report>,
) -> eyre::Result<Data> {
	poise::builtins::register_globally(context, &framework.options().commands).await?;

	let postgres_config = std::env::var("POSTGRES_CONNECTION")?;

	let config_path = std::env::var("CONFIG_PATH").unwrap_or("config.toml".into());

	let config = Config::new_from_file(Path::new(&config_path))?;

	context
		.shard
		.set_activity(Some(serenity::ActivityData::custom(
			&config.messages.status,
		)));

	let (mut pg_client, connection) =
		tokio_postgres::connect(&postgres_config, tokio_postgres::NoTls).await?;

	tokio::spawn(async move {
		connection.await.expect("Postgres connection error");
	});

	migrations::runner().run_async(&mut pg_client).await?;

	Ok(Data {
		config,
		pg: pg_client,
	})
}

async fn handle_shutdown(shard_manager: Arc<serenity::ShardManager>, reason: ShutdownReason) {
	let reason = match reason {
		ShutdownReason::CtrlC => "Interrupted",
		ShutdownReason::Sigterm => "Received SIGTERM",
	};

	warn!("{reason}! Shutting down bot...");
	shard_manager.shutdown_all().await;
	println!("Everything is shutdown. Goodbye!");
}
