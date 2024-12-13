use close::aclose;
use close::close;
use delete::delete;
use edit::edit;
use reply::areply;
use reply::reply;

mod close;
mod common;
mod delete;
mod edit;
mod reply;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![reply(), areply(), delete(), edit(), close(), aclose()]
}
