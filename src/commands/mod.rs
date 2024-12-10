use close::close;

mod close;
mod common;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![close()]
}
