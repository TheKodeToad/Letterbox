use close::aclose;
use close::close;
use contact::contact;
use delete::delete;
use edit::edit;
use reply::areply;
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
		areply(),
		delete(),
		edit(),
		close(),
		aclose(),
		contact(),
	]
}
