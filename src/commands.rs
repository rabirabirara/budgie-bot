use crate::util::{
    check,
    DEFAULT_AUDIO_FILENAME,
};
use gtts;
use serenity::{
    framework::standard::{macros::{command, hook}, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};


fn fetch_gtts(s: &str) -> bool {
    gtts::save_to_file(s, DEFAULT_AUDIO_FILENAME)
}


#[derive(Debug)]
pub struct WhoseParrot;

impl TypeMapKey for WhoseParrot {
    type Value = Option<UserId>;
}

#[command]
pub async fn parrot(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    // bot should first check if in channel
    let manager = songbird::get(&ctx).await.ok_or("songbird client inits at start")?;
    if manager.get(guild_id).is_none() {
        check(msg.channel_id.say(&ctx.http, "I need to be in a voice channel for this to work!  Use `!join`.").await);
        return Ok(());
    }
    
    // check if parroting is active/inactive
    let mut typemap = ctx.data.write().await;
    let parrot_active = typemap.get::<WhoseParrot>().expect("should have initialized WhoseParrot with value of None, so that it exists in typemap").is_some();
    if parrot_active {
        // toggle parrot off
        typemap.insert::<WhoseParrot>(None);
        check(msg.channel_id.say(&ctx.http, "Parroting off.  Bu-kaw!").await);
    } else {
        // toggle parrot on and set Some(userid)
        typemap.insert::<WhoseParrot>(Some(msg.author.id));
        check(msg.channel_id.say(&ctx.http, "Success! I will now parrot everything you say.").await);
    }
    Ok(())
}

#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    // data is a TypeMap.  it basically is a map, with types as keys to unique values.
    // why? well, it may be more useful to think of the value of a key in a TypeMap as
    // its field, like a type constructor.
    let data = ctx.data.read().await;
    if let Some(userid) = data.get::<WhoseParrot>().expect("should have WhoseParrot initialized") {
        if &msg.author.id == userid {
            // try to tts the message
            if !fetch_gtts(&msg.content) {
                eprintln!("failed to get GTTS audio and save it.");
                check(msg.channel_id.say(&ctx.http, format!("failed to say: {}", msg.content)).await);
            } else {
                say_last_audio(ctx, msg).await;
            }
        }
        // simply ignore messages from people who don't own the parrot, and if the parrot isn't active
    }
}

pub async fn say_last_audio(ctx: &Context, msg: &Message) -> CommandResult {
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
    if !fetch_gtts(args.message()) {
        eprintln!("failed to get GTTS audio and save it.");
        return Ok(());
    }
    
    say_last_audio(ctx, msg).await
}

#[command]
pub async fn repeat(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    say_last_audio(ctx, msg).await
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
