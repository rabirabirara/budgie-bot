mod commands;
mod util;

use crate::util::*;
use crate::commands::*;
use std::fs;
use std::path::Path;
use dotenv;
use songbird::SerenityInit;
use serenity::client::Context;
use serenity::prelude::*;
use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::group,
        },
        StandardFramework,
    },
    model::{
        gateway::Ready,
        channel::Message,
    }
};


struct Handler;

#[async_trait] // just a trait with async fns
impl EventHandler for Handler {
    // write a handler that fires when the bot starts up - specifically, on a "ready" signal from discord.
    // the context is passed in but not really necessary.  instead, the ready struct is useful.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// Create a GENERAL_GROUP of commands.
#[group]
#[commands(join, leave, say, repeat, echo, whisper, parrot)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("should have the .env file in the environment");
    let token = dotenv::var("DISCORD_TOKEN").expect("correct the .env file with the discord token!");
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix("!")
            //.owners()
        )
        .normal_message(normal_message)
        .group(&GENERAL_GROUP);

    // intents are just events that your bot should receive.
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;
    
    // the Client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<WhoseParrot>(None)    // initialize parrot
        .register_songbird()
        .await
        .expect("Err creating client");
    
    // set up ctrl-c handler
    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });
    
    let _ = tokio::signal::ctrl_c().await;
    
    // delete the last audio file
    if Path::new(DEFAULT_AUDIO_FILENAME).exists() {
        if let Err(e) = fs::remove_file(DEFAULT_AUDIO_FILENAME) {
            println!("Failed to remove audio file: {e}");
        } else {
            println!("Successfully removed last audio file.")
        }
    } else {
        println!("No audio file was or is present.")
    }
    
    // TODO: leave the voice channel, if in one right now
    println!("Received Ctrl-C, shutting down.");    
}
