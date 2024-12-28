use poise::serenity_prelude as serenity;

pub const UNKNOWN_CHANNEL: isize = 10_003;
pub const CANNOT_MESSAGE: isize = 50_007;

pub fn get_json_error_code(error: &serenity::Error) -> Option<isize> {
	if let serenity::Error::Http(serenity::HttpError::UnsuccessfulRequest(
		serenity::ErrorResponse {
			error: serenity::DiscordJsonError { code, .. },
			..
		},
	)) = error
	{
		Some(*code)
	} else {
		None
	}
}
