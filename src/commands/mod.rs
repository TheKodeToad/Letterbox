mod block;
mod close;
mod contact;
mod delete;
mod edit;
mod info;
mod manage_tags;
mod reply;
mod unblock;
mod util;

pub fn commands() -> Vec<poise::Command<crate::Data, eyre::Error>> {
	vec![
		info::info(),
		reply::reply(),
		reply::anon_reply(),
		reply::tag_reply(),
		reply::anon_tag_reply(),
		delete::delete(),
		delete::delete_context_menu(),
		edit::edit(),
		edit::edit_context_menu(),
		close::close(),
		close::anon_close(),
		close::silent_close(),
		contact::contact(),
		block::block(),
		block::silent_block(),
		unblock::unblock(),
		unblock::silent_unblock(),
		manage_tags::set_tag(),
		manage_tags::delete_tag(),
	]
}
