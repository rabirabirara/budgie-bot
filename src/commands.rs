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

async fn say_last_audio(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    
    let manager = songbird::get(&ctx).await.ok_or("songbird client inits at start")?;
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
    } else {
        check(msg.channel_id.say(&ctx.http, "I need to be in a voice channel for this to work!  Use `!join`.").await);
    }
    Ok(())
}

#[command]
pub async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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
pub async fn leave(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    
    let manager = songbird::get(&ctx).await.ok_or("songbird client missing")?;
    manager.remove(guild_id).await?;
    Ok(())
}


#[command]
pub async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !gtts::save_to_file(args.message(), DEFAULT_AUDIO_FILENAME) {
        eprintln!("failed to get GTTS audio and save it.");
        return Ok(());
    }
    
    say_last_audio(ctx, msg, args).await
}

#[command]
pub async fn repeat(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    say_last_audio(ctx, msg, args).await
}

#[command]
pub async fn echo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    check(msg.channel_id.say(&ctx.http, args.message()).await);
    Ok(())    
}

#[command]
pub async fn whisper(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    check(msg.author.dm(&ctx, |m| m.content(args.message())).await);
    Ok(())    
}
