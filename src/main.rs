use std::env;

use serenity::prelude::*;
use serenity::{
    model::{
        gateway::Ready,
        channel::Message
    },
    async_trait,
};

struct Handler;

#[async_trait]  // just a trait with async fns
impl EventHandler for Handler {     // from prelude
    // write a handler that fires whenever a message, any message, is received.
    // yes, this means discord botes technically read every single message sent! of course they have to, just like how voice assistants listen to everything you say.
    async fn message(&self, ctx: Context, msg: Message) {
        // it reads a message from the discord context; wherever it is reading messages from I bet
        if msg.content == "!ping" {
            // now, try to say "Pong" back; if it fails, then server prints error to terminal
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                eprintln!("Error sending message: {:?}", why);
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