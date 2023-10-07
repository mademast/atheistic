use std::env;

use none_of_those_words_are_in_the_bible::are_any_of_these_words_in_the_bible;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("message received: {}", msg.content);
        if msg.content == "!ping" {
            let res = msg.channel_id.say(&ctx.http, "Pong!").await;
            if let Err(why) = res {
                println!("Error sending message: {:?}", why);
            }
        } else if !are_any_of_these_words_in_the_bible(&msg.content) {
            let res = msg
                .channel_id
                .say(&ctx.http, "none of those words are in the bible")
                .await;
            if let Err(why) = res {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
