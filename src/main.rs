#![feature(result_option_inspect)]
use std::env;

use serenity::prelude::*;
use serenity::{
    model::{
        gateway::Ready,
        channel::Message
    },
    utils::MessageBuilder,
    async_trait,
};

struct Handler;

#[async_trait]  // just a trait with async fns
impl EventHandler for Handler {     // from prelude
    // write a handler that fires whenever a message, any message, is received.
    // yes, this means discord botes technically read every single message sent! of course they have to, just like how voice assistants listen to everything you say.
    async fn message(&self, ctx: Context, msg: Message) {
        let mut command = msg.content;
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
                "!echo" | "!say" => {
                    if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
                        eprintln!("err: {e}");
                    }
                }
                "!whisper" => {
                    if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
                        eprintln!("err: {e}");
                    }
                    if let Err(e) = msg.author.dm(&ctx, |m| m.content(text)).await {
                        eprintln!("err: {e}");
                    }
                }
                _ => (),
            }
        }
    }

    // write a handler that fires when the bot starts up - specifically, on a "ready" signal from discord.
    // the context is passed in but not really necessary.  instead, the ready struct is useful.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // intents are just events that your bot should receive.
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    // the Client 
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // start a shard and start the bot listening to events!
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}