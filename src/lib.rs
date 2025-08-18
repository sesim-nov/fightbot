use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use poise::serenity_prelude as serenity;

pub mod commands;
pub mod pvp_fight;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct FightId {
    guild_id: serenity::GuildId,
    size: usize,
}

pub struct Data {
    pub queues: Arc<Mutex<HashMap<FightId, HashSet<serenity::UserId>>>>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
