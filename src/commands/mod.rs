use block::block;
use block::silent_block;
use close::anon_close;
use close::close;
use close::silent_close;
use contact::contact;
use delete::delete;
use delete::delete_context_menu;
use edit::edit;
use edit::edit_context_menu;
use reply::anon_reply;
use reply::reply;
use unblock::silent_unblock;
use unblock::unblock;

mod block;
mod close;
mod contact;
mod delete;
mod edit;
mod reply;
mod unblock;
mod util;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![
		reply(),
		anon_reply(),
		delete(),
		delete_context_menu(),
		edit(),
		edit_context_menu(),
		close(),
		anon_close(),
		silent_close(),
		contact(),
		block(),
		silent_block(),
		unblock(),
		silent_unblock(),
	]
}
