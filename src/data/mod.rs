use refinery::embed_migrations;

pub mod blocked_users;
pub mod received_messages;
pub mod sent_messages;
pub mod threads;
pub mod tags;

embed_migrations!();
