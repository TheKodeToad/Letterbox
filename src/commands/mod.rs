use close::anon_close;
use close::close;
use close::silent_close;
use contact::contact;
use delete::delete;
use edit::edit;
use reply::anon_reply;
use reply::reply;

mod close;
mod contact;
mod delete;
mod edit;
mod reply;
mod util;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![
		reply(),
		anon_reply(),
		delete(),
		edit(),
		close(),
		anon_close(),
		silent_close(),
		contact(),
	]
}
