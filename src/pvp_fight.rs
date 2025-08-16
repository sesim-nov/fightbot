use std::collections::HashSet;

use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateButton, CreateEmbed, Mentionable, Team, UserId,
};
use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::Error;

pub mod result;
use result::*;

pub enum FightState {
    RegistrationOpen,
    Started,
    Complete,
    Canceled,
}

pub enum FightKind {
    Casual,
    Ranked,
}

pub struct PVPTeams(Vec<UserId>, Vec<UserId>);

pub struct PVPFight {
    id: Uuid,
    pool_size: usize,
    team_pool: HashSet<UserId>,
    fight_state: FightState,
    fight_kind: FightKind,
    teams: Option<PVPTeams>,
    votes: Option<Votes>,
}

impl PVPFight {
    /// Instance a new fight
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team_size(mut self, ts: usize) -> Self {
        self.pool_size = ts * 2;
        self
    }

    pub fn fight_kind(mut self, fk: FightKind) -> Self {
        self.fight_kind = fk;
        self
    }

    pub fn set_state(&mut self, state: FightState) {
        self.fight_state = state;
    }

    /// Register a participant
    pub fn reg(&mut self, add_id: UserId) -> Result<(), Error> {
        if self.pool_size <= self.team_pool.len() {
            Err("Team Full".into())
        } else {
            self.team_pool.insert(add_id);
            Ok(())
        }
    }

    /// Remove a partiipant
    pub fn rm(&mut self, rm_id: &UserId) -> bool {
        self.team_pool.remove(rm_id)
    }

    /// Generate 2 random teams from the current pool.
    fn get_teams(&self) -> PVPTeams {
        let mut rng = rand::thread_rng();
        let mut combatants: Vec<UserId> = self.team_pool.iter().map(|i| i.to_owned()).collect();
        combatants.shuffle(&mut rng);
        let center = combatants.len() / 2;
        let other_combatants = combatants.split_off(center);
        PVPTeams(combatants, other_combatants)
    }

    /// Get a list of current participants as a newline separated string.
    fn get_pool_list(&self) -> String {
        to_mention_string(self.team_pool.iter().collect())
    }

    // Check if we're ready to start a match.
    pub fn ready_to_start(&self) -> bool {
        self.pool_size <= self.team_pool.len()
    }

    pub fn closed(&self) -> bool {
        match self.fight_kind {
            FightKind::Casual => match self.fight_state {
                FightState::RegistrationOpen => false,
                _ => true,
            },
            FightKind::Ranked => match self.fight_state {
                FightState::RegistrationOpen => false,
                FightState::Started => false,
                _ => true,
            },
        }
    }   

    pub fn cast_vote(&mut self, user: UserId, vote: TeamName) {
        todo!();
    }

    // Generate and embed showing the progress of this PVP fight
    fn get_progress_embed(&self) -> CreateEmbed {
        let team_size = self.pool_size / 2;
        let team_names = self.get_pool_list();
        CreateEmbed::new().fields(vec![
            (
                format!("PvP Match: {team_size}v{team_size}"),
                "Welcome to this PVP Match.",
                false,
            ),
            (
                format!(
                    "Registered CMDRs ({}/{})",
                    self.team_pool.len(),
                    self.pool_size
                ),
                &team_names,
                true,
            ),
            (
                "".to_string(),
                "Use the buttons below to manage this match. You have 5 minutes to start the fight!",
                false,
            ),
        ])
    }

    // Get the embed that lists the details for a match ready to start.
    fn get_start_embed(&self) -> CreateEmbed {
        let team_size = self.pool_size / 2;

        let PVPTeams(team_a, team_b) = self.get_teams();
        let team_a = to_mention_string(team_a.iter().collect());
        let team_b = to_mention_string(team_b.iter().collect());

        CreateEmbed::new().fields(vec![
            (
                "Fight Start",
                format!("{team_size}v{team_size} fight has Started!"),
                false,
            ),
            ("Team A", team_a, true),
            ("Team B", team_b, true),
        ])
    }

    // Cancel the fight and return a blank embed.
    fn get_cancel_embed(&self) -> CreateEmbed {
        CreateEmbed::new().field("Fight Cancelled", "Fight has been cancelled", false)
    }

    // Cancel the fight and return a blank embed.
    fn get_complete_embed(&self) -> CreateEmbed {
        CreateEmbed::new().field(
            "Fight Complete",
            "Fight has been closed and recorded",
            false,
        )
    }

    /// Get control buttons for registration.
    fn get_reg_buttons(&self) -> Vec<CreateActionRow> {
        let buttons = vec![
            CreateButton::new("reg").label("Join"),
            CreateButton::new("rm").label("Leave"),
            CreateButton::new("start")
                .label("Start Match")
                .style(serenity::ButtonStyle::Danger),
            CreateButton::new("cancel")
                .label("Cancel")
                .style(serenity::ButtonStyle::Danger),
        ];
        vec![CreateActionRow::Buttons(buttons)]
    }

    fn get_result_buttons(&self) -> Vec<CreateActionRow> {
        let buttons = vec![
            CreateButton::new("a_wins").label("Vote Team A"),
            CreateButton::new("b_wins").label("Vote Team B"),
            CreateButton::new("cancel").label("Cancel Fight"),
        ];
        vec![CreateActionRow::Buttons(buttons)]
    }
}

impl std::default::Default for PVPFight {
    fn default() -> Self {
        Self {
            team_pool: HashSet::new(),
            id: uuid::Uuid::new_v4(),
            pool_size: 2,
            fight_state: FightState::RegistrationOpen,
            fight_kind: FightKind::Casual,
            teams: None,
            votes: None,
        }
    }
}

impl From<&PVPFight> for CreateEmbed {
    fn from(fight: &PVPFight) -> Self {
        match fight.fight_state {
            FightState::RegistrationOpen => fight.get_progress_embed(),
            FightState::Started => fight.get_start_embed(),
            FightState::Canceled => fight.get_cancel_embed(),
            FightState::Complete => fight.get_complete_embed(),
        }
    }
}

impl From<&PVPFight> for Vec<CreateActionRow> {
    fn from(fight: &PVPFight) -> Self {
        match fight.fight_state {
            FightState::RegistrationOpen => fight.get_reg_buttons(),
            FightState::Started => match fight.fight_kind {
                FightKind::Casual => Vec::new(),
                FightKind::Ranked => fight.get_result_buttons(),
            },
            _ => Vec::new(),
        }
    }
}

/// Convert a vector of UserIDs into a newline-separated string of mentions.
fn to_mention_string(id_list: Vec<&UserId>) -> String {
    id_list
        .iter()
        .map(|x| x.mention().to_string())
        .collect::<Vec<String>>()
        .join("\n")
}
