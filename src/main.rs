use core::error;
use std::path::Path;

use commands::commands;
use config::Config;
use data::migrations;
use error_handler::handle_error;
use event_handlers::handle_event;
use poise::{
	serenity_prelude::{self as serenity, ActivityData},
	PrefixFrameworkOptions,
};

mod commands;
mod common;
mod config;
mod data;
mod error_handler;
mod event_handlers;

pub struct Data {
	config: Config,
	pg: tokio_postgres::Client,
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
		prefix_options: PrefixFrameworkOptions {
			dynamic_prefix: Some(|context| {
				Box::pin(async move { Ok(Some(context.data.config.prefix.to_owned())) })
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

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

	let mut client = serenity::ClientBuilder::new(bot_token, intents)
		.framework(framework)
		.await?;

	client.start().await?;

	Ok(())
}

async fn setup(
	context: &serenity::Context,
	framework: &poise::Framework<Data, eyre::Report>,
) -> eyre::Result<Data> {
	poise::builtins::register_globally(context, &framework.options().commands).await?;

	let postgres_config = std::env::var("POSTGRES_CONNECTION")?;

	let config = Config::new_from_file(Path::new("config.toml"))?;
	context
		.shard
		.set_activity(Some(ActivityData::custom(&config.status)));

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
