use refinery::embed_migrations;

pub mod received_messages;
pub mod sent_messages;
pub mod threads;
pub mod blocked_users;

embed_migrations!();
