use poise::serenity_prelude::{self as serenity, GuildId};

use crate::util::markdown;

const HEADER: &str = "↷ *Forwarded*";

pub async fn create(
	context: &serenity::Context,
	message: &serenity::Message,
	receiving_guild_id: GuildId,
) -> Option<String> {
	let Some(serenity::MessageReference {
		kind: serenity::MessageReferenceKind::Forward,
		guild_id,
		channel_id,
		message_id: Some(message_id),
		..
	}) = message.message_reference
	else {
		return None;
	};

	let Some(snapshot) = message.message_snapshots.first() else {
		return None;
	};

	let footer = if guild_id == Some(receiving_guild_id) {
		if let Some(receiving_guild) = receiving_guild_id.to_guild_cached(&context) {
			if let Some(channel) = receiving_guild.channels.get(&channel_id) {
				let name = markdown::escape(&channel.name);

				Some(match channel.kind {
					serenity::ChannelType::Text | serenity::ChannelType::News => format!("#{name}"),
					_ => name,
				})
			} else if let Some(thread) = receiving_guild
				.threads
				.iter()
				.find(|thread| thread.id == channel_id)
			{
				Some(markdown::escape(&thread.name))
			} else {
				Some("#*Unknown*".to_string())
			}
		} else {
			None
		}
	} else {
		if let Some(guild_id) = guild_id {
			guild_id
				.to_guild_cached(&context)
				.map(|guild| markdown::escape(&guild.name))
				.or_else(|| Some("Unknown Server".to_string()))
		} else {
			None
		}
	};

	let content = if !snapshot.content.is_empty() {
		&snapshot.content
	} else {
		"*No content*"
	};

	if let Some(footer) = footer {
		Some(markdown::quote(&format!(
			"-# {HEADER}\n{}\n-# [{} • {} ›]({})",
			content,
			footer,
			serenity::FormattedTimestamp::new(
				message_id.created_at(),
				Some(serenity::FormattedTimestampStyle::ShortDateTime)
			),
			message_id.link(channel_id, guild_id),
		)))
	} else {
		Some(markdown::quote(&format!("-# {HEADER}\n{}", content)))
	}
}
