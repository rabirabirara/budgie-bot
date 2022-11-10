use serenity::Result as SerenityResult;
use serenity::model::channel::Message;

pub const DEFAULT_AUDIO_FILENAME: &'static str = "current_speak.mp3";

pub fn check(result: SerenityResult<Message>) {
    if let Err(e) = result {
        eprintln!("Err: {e}");
    }
}
