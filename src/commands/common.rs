use crate::Data;

pub type Context<'a> = poise::Context<'a, Data, eyre::Report>;

pub async fn require_moderator(context: Context<'_>) -> eyre::Result<bool> {
	let Some(member) = context.author_member().await else {
		return Ok(false);
	};

	Ok(context.data().config.is_moderator(&member.roles))
}
