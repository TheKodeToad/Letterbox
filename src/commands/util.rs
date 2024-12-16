use crate::Data;

pub type Context<'a> = poise::Context<'a, Data, eyre::Report>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, Data, eyre::Report>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, eyre::Report>;

pub async fn require_staff(context: Context<'_>) -> eyre::Result<bool> {
	let Some(member) = context.author_member().await else {
		return Ok(false);
	};

	Ok(context.data().config.is_staff(&member.roles))
}
