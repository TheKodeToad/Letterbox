use close::close;
use delete::delete;
use reply::areply;
use reply::reply;

mod close;
mod common;
mod reply;
mod delete;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![reply(), areply(), delete(), close()]
}
