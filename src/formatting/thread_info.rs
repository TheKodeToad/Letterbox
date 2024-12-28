use std::fmt::Write;

use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::config::Config;

pub struct ThreadInfoOptions {
	pub user_id: serenity::UserId,
	pub opened: (serenity::UserId, serenity::Timestamp),
	pub closed: Option<(serenity::UserId, serenity::Timestamp)>,
}

pub fn make_thread_info(config: &Config, options: ThreadInfoOptions) -> String {
	let opened_discord_timestamp = serenity::FormattedTimestamp::new(
		options.opened.1,
		Some(serenity::FormattedTimestampStyle::RelativeTime),
	);

	let mut result = String::new();

	if let Some(role) = config.forum_channel.mention_role_id {
		writeln!(&mut result, "{}\n", role.mention()).unwrap();
	}

	if options.user_id == options.opened.0 {
		writeln!(
			&mut result,
			"📩 Thread opened by {} {}",
			options.user_id.mention(),
			opened_discord_timestamp
		)
		.unwrap();
	} else {
		writeln!(
			&mut result,
			"📩 Thread for {} opened by {} {}",
			options.user_id.mention(),
			options.opened.0.mention(),
			opened_discord_timestamp
		)
		.unwrap();
	}

	if let Some(closed_by) = options.closed {
		let closed_discord_timestamp = serenity::FormattedTimestamp::new(
			closed_by.1,
			Some(serenity::FormattedTimestampStyle::RelativeTime),
		);

		writeln!(
			&mut result,
			"⛔ Thread closed by {} {}",
			serenity::Mention::User(closed_by.0),
			closed_discord_timestamp
		).unwrap();
	}

	result
}

pub fn make_thread_info_allowed_mentions(config: &Config) -> serenity::CreateAllowedMentions {
	if let Some(role) = config.forum_channel.mention_role_id {
		serenity::CreateAllowedMentions::new().roles([role])
	} else {
		serenity::CreateAllowedMentions::new()
	}
}