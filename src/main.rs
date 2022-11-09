mod commands;
mod util;

use crate::util::check;
use crate::commands::*;
use std::env;

// songbird imports
use songbird::SerenityInit;

use serenity::client::Context;

use serenity::prelude::*;
use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    utils::MessageBuilder,
    Result as SerenityResult,
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
#[commands(join, say, whisper, tts)]
struct General;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    // intents are just events that your bot should receive.
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;
    
    // the Client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");
    
    // set up ctrl-c handler
    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });
    
    let _ = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");    
}
