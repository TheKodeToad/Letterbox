use close::close;
use delete::delete;
use reply::areply;
use reply::reply;

mod close;
mod common;
mod delete;
mod reply;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![reply(), areply(), delete(), close()]
}
