use std::collections::HashSet;

use poise::serenity_prelude::{CreateEmbed, Mentionable, UserId};
use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::Error;

pub enum FightState {
    RegistrationOpen,
    Started,
    //Complete, <- Future state, to be used for ranked peeveepee
    Canceled,
}

pub struct PVPTeams(Vec<UserId>, Vec<UserId>);

pub struct PVPFight {
    id: Uuid,
    pool_size: usize,
    team_pool: HashSet<UserId>,
    fight_state: FightState,
}

impl PVPFight {
    /// Instance a new fight
    pub fn new(team_size: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            pool_size: 2 * team_size,
            team_pool: HashSet::new(),
            fight_state: FightState::RegistrationOpen,
        }
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
        if let FightState::RegistrationOpen = self.fight_state {
            false
        } else {
            true
        }
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
}

impl From<&PVPFight> for CreateEmbed {
    fn from(fight: &PVPFight) -> Self {
        match fight.fight_state {
            FightState::RegistrationOpen => fight.get_progress_embed(),
            FightState::Started => fight.get_start_embed(),
            FightState::Canceled => fight.get_cancel_embed(),
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
