use refinery::embed_migrations;

pub mod received_messages;
pub mod threads;
pub mod sent_messages;

embed_migrations!();
