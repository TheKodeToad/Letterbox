use crate::{data::tags, Data};

pub type Context<'a> = poise::Context<'a, Data, eyre::Report>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, Data, eyre::Report>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, eyre::Report>;

pub async fn require_staff(context: Context<'_>) -> eyre::Result<bool> {
	let Some(member) = context.author_member().await else {
		return Ok(false);
	};

	Ok(context
		.data()
		.config
		.staff_roles
		.iter()
		.any(|role| member.roles.contains(role)))
}

pub async fn complete_tags(context: Context<'_>, partial: &str) -> Vec<String> {
	match tags::search(&context.data().pg, partial).await {
		Ok(tags) => tags,
		Err(error) => {
			log::error!("Error completing tags: {error:?}");
			vec![]
		}
	}
}
