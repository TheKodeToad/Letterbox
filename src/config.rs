use poise::serenity_prelude as serenity;
use std::{collections::HashSet, fs, path::Path};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
	pub server_id: serenity::GuildId,
	#[serde(default = "staff_roles_default" /* [] */)]
	pub staff_roles: HashSet<serenity::RoleId>,
	#[serde(default = "prefix_default" /* = */)]
	pub prefix: String,
	pub forum_channel: ForumChannelConfig,
	#[serde(default)]
	pub messages: MessageConfig,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumChannelConfig {
	pub id: serenity::ChannelId,
	pub open_tag_id: Option<serenity::ForumTagId>,
	pub closed_tag_id: Option<serenity::ForumTagId>,
	pub mention_role_id: Option<serenity::RoleId>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageConfig {
	pub status: String,
	pub anonymous_reply_title: String,
	pub thread_open: Option<String>,
	pub thread_closed: Option<String>,
}

impl Config {
	pub fn new_from_file(path: &Path) -> eyre::Result<Config> {
		let data = fs::read_to_string(path)?;
		Ok(toml::from_str(&data)?)
	}

	pub fn is_staff(&self, roles: &[serenity::RoleId]) -> bool {
		roles.iter().any(|role| self.staff_roles.contains(role))
	}
}

impl Default for MessageConfig {
	fn default() -> Self {
		Self { status: "Message me to contact staff!".to_string(), anonymous_reply_title: "Staff Member".to_string(), thread_open: None, thread_closed: None }
	}
}

impl ForumChannelConfig {
	pub fn allowed_mentions(&self) -> serenity::CreateAllowedMentions {
		if let Some(role) = self.mention_role_id {
			serenity::CreateAllowedMentions::new().roles([role])
		} else {
			serenity::CreateAllowedMentions::new()
		}
	}
}

// https://github.com/serde-rs/serde/issues/368

fn staff_roles_default() -> HashSet<serenity::RoleId> {
	HashSet::new()
}

fn prefix_default() -> String {
	"=".to_string()
}
