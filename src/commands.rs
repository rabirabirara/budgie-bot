use crate::util::{
    check,
    DEFAULT_AUDIO_FILENAME,
};
use gtts;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
pub async fn join(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    if let Some(channel_id) = guild
        .voice_states 
        .get(&msg.author.id) 
        .and_then(|vst| vst.channel_id) 
    {
        let manager = songbird::get(&ctx).await.ok_or("songbird client missing")?;
        let _handler = manager.join(guild_id, channel_id).await;
    } else {
        check(msg.channel_id.say(&ctx.http, "You need to be in a voice channel for this to work!").await);
    }
    
    Ok(())
}

#[command]
pub async fn leave(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    
    let manager = songbird::get(&ctx).await.ok_or("songbird client missing")?;
    manager.remove(guild_id).await?;
    Ok(())
}

#[command]
pub async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: write the msg to a file and then say it.
    if !gtts::save_to_file(args.message(), DEFAULT_AUDIO_FILENAME) {
        eprintln!("failed to get GTTS audio and save it.");
        return Ok(());
    }
    
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    
    let manager = songbird::get(&ctx).await.expect("songbird client inits at start");
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let source = match songbird::ffmpeg(DEFAULT_AUDIO_FILENAME).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("failed to play audio from file: {e}");
                return Ok(());
            }
        };
        
        handler.play_source(source);        
        // msg.say playing
    } else {
        // TODO: msg.say this
        eprintln!("Not in a voice channel");
    }
    Ok(())
}

#[command]
pub async fn echo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    check(msg.channel_id.say(&ctx.http, args.message()).await);
    Ok(())    
}

#[command]
pub async fn whisper(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    check(msg.author.dm(&ctx, |m| m.content(args.message())).await);
    Ok(())    
}

#[command]
pub async fn tts(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    check(msg.channel_id.say(&ctx.http, "unimplemented!").await);
    Ok(())
}

/*
    async fn message(&self, ctx: Context, msg: Message) {
        let mut command = msg.content.clone();
        if let Some(i) = command.find(' ') {
            let text = command.split_off(i);
            let response = MessageBuilder::new()
                .push_bold_safe(&msg.author.name)
                .push(" used ")
                .push_mono_safe(&command)
                .push(": ")
                .quote_rest()
                .push(text.clone())
                .build();

            match command.trim() {
                "!join" => {
                    if let Some(guild) = msg.guild(&ctx.cache) {
                        let user_id = msg.author.id;
                        if let Some(cid) = guild.voice_states.get(&user_id).and_then(|vst| vst.channel_id) {
                            // join the channel id
                            let manager = songbird::get(&ctx).await
                                .expect("Songbird client should've been placed in at init.").clone();
                            let _handler = manager.join(guild.id, cid).await;
                        } else {
                            check(msg.channel_id.say(&ctx.http, "You must be in a voice channel to use this feature!").await);
                        }
                    }
                }
                "!echo" | "!say" => {
                    check(msg.channel_id.say(&ctx.http, response).await);
                }
                "!whisper" => {
                    check(msg.channel_id.say(&ctx.http, response).await);
                    check(msg.author.dm(&ctx, |m| m.content(text)).await);
                }
                _ => (),
            }
        }
    }


*/
