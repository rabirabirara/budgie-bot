use serenity::Result as SerenityResult;
use serenity::model::channel::Message;

pub fn check(result: SerenityResult<Message>) {
    if let Err(e) = result {
        eprintln!("Err: {e}");
    }
}
