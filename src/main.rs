use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use dotenv::dotenv;
use poise::serenity_prelude as serenity;

mod commands;

#[derive(Clone, Eq, PartialEq, Hash)]
struct FightId {
    guild_id: serenity::GuildId,
    size: usize,
}

struct Data {
    queues: Arc<Mutex<HashMap<FightId, HashSet<serenity::UserId>>>>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    //intents.insert(serenity::GatewayIntents::MESSAGE_CONTENT); <- uncomment to add message content parsing for slash commands.

    let prefix_options = poise::PrefixFrameworkOptions {
        prefix: Some("+".to_string()),
        ..Default::default()
    };

    let options = poise::FrameworkOptions {
        commands: vec![commands::reg()],
        pre_command: |ctx| {
            Box::pin(async move { println!("Executing command: {}", ctx.command().qualified_name) })
        },
        prefix_options: prefix_options,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    queues: Arc::new(Mutex::new(HashMap::new())),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
