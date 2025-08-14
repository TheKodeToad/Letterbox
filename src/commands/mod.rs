mod general;
mod meta;
mod thread;
mod util;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![
		meta::info::info(),
		meta::help::help(),
		general::block::block(),
		general::block::silent_block(),
		general::unblock::unblock(),
		general::unblock::silent_unblock(),
		general::manage_tags::tag_set(),
		general::manage_tags::tag_delete(),
		thread::reply::reply(),
		thread::reply::anon_reply(),
		thread::reply::tag_reply(),
		thread::reply::anon_tag_reply(),
		thread::delete::delete(),
		thread::delete::delete_context_menu(),
		thread::edit::edit(),
		thread::edit::edit_context_menu(),
		thread::close::close(),
		thread::close::anon_close(),
		thread::close::silent_close(),
		thread::contact::contact(),
	]
}
