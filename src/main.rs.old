use std::{collections::HashMap, env, sync::Arc};
use rand::seq::SliceRandom;
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        application::
            Interaction
        ,
        user::User,
    },
    prelude::*,
    all::Message
};

type Queues = Arc<Mutex<HashMap<usize, Vec<User>>>>;

struct Handler {
    queues: Queues,
}

impl Handler {
    async fn register(&self, size: usize, user: User, channel: &serenity::model::id::ChannelId) {
        let mut qs = self.queues.lock().unwrap();
        let q = qs.entry(size).or_default();
        if q.iter().any(|u| u.id == user.id) {
            channel.say(&Context::current().await.http, format!("{} one of you is enough, shitter", user.name)).await.unwrap();
            return;
        }
        q.push(user.clone());
        channel.say(&Context::current().await.http, format!("{} registered for {}‑player group.", user.name, size)).await.unwrap();
        if q.len() >= size {
            let mut team = q.clone();
            team.shuffle(&mut rand::thread_rng());
            let mid = team.len() / 2;
            let (t1, t2) = team.split_at(mid);
            let t1_names: String = t1.iter().map(|u| &u.name).collect::<Vec<_>>().join("\n");
            let t2_names: String = t2.iter().map(|u| &u.name).collect::<Vec<_>>().join("\n");
            channel.say(&Context::current().await.http,
                format!("{}‑player group filled! 🛡️\n**Team 1**:\n{}\n\n**Team 2**:\n{}",
                        size, t1_names, t2_names)
            ).await.unwrap();
            q.clear();
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.trim();
        if content == "?r" {
            msg.reply(&ctx.http, "Use `/2v2`, `/3v3`, or `/4v4` to register.").await.unwrap();
        } else if content == "?cancel" {
            let mut qs = self.queues.lock().unwrap();
            let mut removed = false;
            for (&sz, q) in qs.iter_mut() {
                if let Some(pos) = q.iter().position(|u| u.id == msg.author.id) {
                    q.remove(pos);
                    msg.reply(&ctx.http, format!("{} canceled from the {}-player queue.", msg.author.name, sz)).await.unwrap();
                    removed = true;
                    break;
                }
            }

            if !removed {
                msg.reply(&ctx.http, "You are not in any queue").await.unwrap();
            }
        } else if content.starts_with("?cancel ") {
            // Handle specific group size cancel
            let mut qs = self.queues.lock().unwrap();
            if let Some(queue_size) = content.split_whitespace().nth(1) {
                let size = match queue_size {
                    "2v2" => 4,
                    "3v3" => 6,
                    "4v4" => 8,
                    _ => return,
                };
                if let Some(q) = qs.get_mut(&size) {
                    q.clear();
                    msg.reply(&ctx.http, format!("The {}-player queue has been cleared.", size)).await.unwrap();
                }
            }
        } else if content == "?start" {
            let mut qs = self.queues.lock().unwrap();
            for (&sz, q) in qs.iter_mut() {
                if q.len() >= 2 {
                    let mut team = q.clone();
                    team.shuffle(&mut rand::thread_rng());
                    let mid = team.len()/2;
                    let (t1, t2) = team.split_at(mid);
                    let t1_names = t1.iter().map(|u| &u.name).collect::<Vec<_>>().join("\n");
                    let t2_names = t2.iter().map(|u| &u.name).collect::<Vec<_>>().join("\n");
                    msg.channel_id.say(&ctx.http,
                        format!("Manually starting {}-player group:\n**Team 1**:\n{}\n\n**Team 2**:\n{}",
                                sz, t1_names, t2_names)
                    ).await.unwrap();
                    q.clear();
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        println!("rebuy time!");
        // Register slash commands
        let cmds = serenity::model::prelude::application_command::ApplicationCommand::create_global_application_command(&ctx.http, |c| {
            c.name("2v2").description("Register for 2v2")
        }).await.unwrap();
        for name in ["3v3","4v4"] {
            serenity::model::prelude::application_command::ApplicationCommand::create_global_application_command(&ctx.http, |c| {
                c.name(name).description(format!("Register for {}", name))
            }).await.unwrap();
        }
        serenity::model::prelude::application_command::ApplicationCommand::create_global_application_command(&ctx.http, |c| {
            c.name("me").description("Remove yourself from the queue you are in")
        }).await.unwrap();
        println!("Slash commands registered.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            let user = cmd.user.clone();
            let channel = cmd.channel_id;
            match cmd.data.name.as_str() {
                "2v2" | "3v3" | "4v4" => {
                    let size: usize = match cmd.data.name.as_str() {
                        "2v2" => 4,
                        "3v3" => 6,
                        "4v4" => 8,
                        _ => return,
                    };
                    self.register(size, user, &channel).await;
                }
                "me" => {
                    // Slash command "/me" to remove from the user's current queue
                    let mut qs = self.queues.lock().unwrap();
                    let mut removed = false;

                    for (&size, users) in qs.iter_mut() {
                        if let Some(pos) = users.iter().position(|u| u.id == user.id) {
                            users.remove(pos);
                            removed = true;
                            break;
                        }
                    }

                    if removed {
                        cmd.create_interaction_response(&ctx.http, |r| {
                            r.kind(InteractionResponseFlagss::ChannelMessageWithSource)
                                .interaction_response_data(|d| d.content("You have been removed from your current queue."))
                        }).await.unwrap();
                    } else {
                        cmd.create_interaction_response(&ctx.http, |r| {
                            r.kind(InteractionResponseFlagss::ChannelMessageWithSource)
                                .interaction_response_data(|d| d.content("You are not in any queue."))
                        }).await.unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing token");
    let queues: Queues = Arc::new(Mutex::new(HashMap::new()));

    let handler = Handler { queues };

    Client::builder(&token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT)
        .event_handler(handler)
        .await
        .expect("Err creating client")
        .start()
        .await
        .expect("Err running client");
}
