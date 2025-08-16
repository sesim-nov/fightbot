use poise::serenity_prelude::UserId;

pub enum TeamName {
    TeamA,
    TeamB,
}

pub struct Votes {
    team_a_vote: Option<TeamName>,
    team_b_vote: Option<TeamName>,
}

impl Votes {
    pub fn new() -> Self {
        Self {
            team_a_vote: None,
            team_b_vote: None,
        }
    }
}

pub struct PVPResult {
    winners: Vec<UserId>,
    losers: Vec<UserId>,
}

impl PVPResult {
    pub fn new(winners: Vec<UserId>, losers: Vec<UserId>) -> Self {
        Self { winners, losers }
    }
}
