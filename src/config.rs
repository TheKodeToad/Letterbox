use poise::serenity_prelude::{self as serenity, RoleId};
use std::{collections::HashSet, fs, path::Path};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
	pub forum_channel_id: serenity::ChannelId,
	pub mention: Option<MentionMode>,
	#[serde(default = "moderator_roles_default" /* [] */)]
	pub moderator_roles: HashSet<serenity::RoleId>,
	#[serde(default = "prefix_default" /* = */)]
	pub prefix: String,
	#[serde(default = "status_default" /* Message me to contact mods! */)]
	pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum MentionMode {
	Everyone,
	Here,
	RoleId(u64),
}

impl Config {
	pub fn new_from_file(path: &Path) -> eyre::Result<Config> {
		let data = fs::read_to_string(path)?;
		Ok(toml::from_str(&data)?)
	}

	pub fn is_moderator(&self, roles: &[RoleId]) -> bool {
		roles.iter().any(|role| self.moderator_roles.contains(role))
	}
}

// https://github.com/serde-rs/serde/issues/368

fn moderator_roles_default() -> HashSet<serenity::RoleId> {
	HashSet::new()
}

fn prefix_default() -> String {
	"=".to_string()
}

fn status_default() -> String {
	"Message me to contact mods!".to_string()
}
