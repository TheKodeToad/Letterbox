use close::close;
use reply::reply;
use reply::areply;

mod close;
mod common;
mod reply;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![reply(), areply(), close()]
}
