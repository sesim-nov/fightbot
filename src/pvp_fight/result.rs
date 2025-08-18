use poise::serenity_prelude::{Team, UserId};

#[derive(Eq, PartialEq, Copy, Clone)]
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
    pub fn set_vote_of(&mut self, voting_team: &TeamName, vote: TeamName) {
        match voting_team {
            TeamName::TeamA => {
                self.team_a_vote = Some(vote);
            }
            TeamName::TeamB => {
                self.team_b_vote = Some(vote);
            }
        }
    }
    pub fn is_unanimous(&self) -> bool {
        if let (Some(a), Some(b)) = (self.team_a_vote.as_ref(), self.team_b_vote.as_ref()) {
            *a == *b
        } else {
            false
        }
    }
    pub fn get_winner(&self) -> Option<TeamName> {
        if self.is_unanimous() {
            Some(self.team_a_vote.unwrap())
        } else {
            None
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
