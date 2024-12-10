use close::close;
use reply::areply;
use reply::reply;

mod close;
mod common;
mod reply;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![reply(), areply(), close()]
}
