use std::fs;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

extern crate regex;
use regex::Regex;

mod bill;

use bill::{edit, query};

struct Handler;

const HELP: &str = "help\n!debt @user1 @user2 100 -> user1 欠 user2 $100\n!pay_back @user1 @user2 100 -> user1 還 user2 $100\n!show_all -> 顯示當前的欠債情形";

#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx: Context, msg: Message) {
        let debt_regex = Regex::new(r"^!debt @(\w+) @(\w+) (\d+)$").unwrap();
        let pay_back_regex = Regex::new(r"^!pay_back @(\w+) @(\w+) (\d+)$").unwrap();
        let show_user = Regex::new(r"^!show @(\w+)$").unwrap();

        if let Some(content) = debt_regex.captures(&msg.content) {
            let user1 = &content[1];
            let user2 = &content[2];
            let amount = &content[3].parse::<i32>().unwrap();

            if let Err(why) = msg.channel_id.say(&ctx.http, edit::debt(user1, user2, *amount)).await {
                println!("Error sending message: {why:?}");
            }
        }
        else if let Some(content) = pay_back_regex.captures(&msg.content) {
            let user1 = &content[1];
            let user2 = &content[2];
            let amount = &content[3].parse::<i32>().unwrap();

            if let Err(why) = msg.channel_id.say(&ctx.http, edit::pay_back(user1, user2, *amount)).await {
                println!("Error sending message: {why:?}");
            }
        }
        else if let Some(content) = show_user.captures(&msg.content) {
            let user = &content[1];

            let text = format!("show {user}");
            if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
                println!("Error sending message: {why:?}");
            }
        }

        else if msg.content == "!show_all" {
            let text = query::show_all();
            if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
                println!("Error sending message: {why:?}");
            }
        }
        else if msg.content == "!help" {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP.to_string()).await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("token.txt").unwrap();
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    
}
