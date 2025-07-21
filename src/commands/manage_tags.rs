use crate::{
	commands::util::{require_staff, Context},
	data::tags::{self},
	util::markdown,
};

/// Set a tag to be used with tag_reply.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("st"),
	ephemeral
)]
pub async fn set_tag(
	context: Context<'_>,
	#[description = "The name to invoke the tag with."]
	#[max_length = 100]
	name: String,
	#[description = "The content to send when invoking the tag. Include \\n to insert a newline."]
	#[max_length = 2000]
	#[rest]
	content: String,
) -> eyre::Result<()> {
	let content = content.replace("\\n", "\n");

	tags::set(&context.data().pg, &name, &content).await?;

	let safe_name = markdown::escape(&name);
	context
		.say(format!(
			"✅ Set tag **{safe_name}**! It can be sent with `tag_reply` or deleted with `delete_tag`."
		))
		.await?;

	Ok(())
}

/// Delete a tag.
#[poise::command(
	slash_command,
	prefix_command,
	guild_only,
	check = "require_staff",
	aliases("dt"),
	ephemeral
)]
pub async fn delete_tag(context: Context<'_>, name: String) -> eyre::Result<()> {
	let deleted = tags::delete(&context.data().pg, &name).await?;

	let safe_name = markdown::escape(&name);
	if deleted {
		context
			.say(format!("✅ Deleted tag **{safe_name}**!"))
			.await?;
	} else {
		context
			.say(format!("❌ Tag **{safe_name}** does not exist."))
			.await?;
	}

	Ok(())
}
