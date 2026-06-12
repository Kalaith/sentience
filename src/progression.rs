//! Morality route math and upgrade scaling.

use crate::state::MoralChoice;

pub const CHOICE_LEVELS: usize = 19;
pub const TOTAL_LEVELS: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampaignRoute {
    Hero,
    Gremlin,
}

#[derive(Debug, Clone, Copy)]
pub struct UpgradeProfile {
    pub savior: usize,
    pub villain: usize,
    pub speed_bonus: f32,
    pub crouch_bonus: f32,
    pub jump_bonus: f32,
    pub stealth_factor: f32,
    pub cloak_duration: f32,
    pub pulse_range: f32,
    pub pulse_damage: i32,
    pub ability_cooldown: f32,
}

impl UpgradeProfile {
    pub fn from_choices(choices: &[MoralChoice]) -> Self {
        let savior = choices
            .iter()
            .filter(|choice| **choice == MoralChoice::Savior)
            .count();
        let villain = choices.len().saturating_sub(savior);

        Self {
            savior,
            villain,
            speed_bonus: savior as f32 * 4.5,
            crouch_bonus: savior as f32 * 6.5,
            jump_bonus: savior as f32 * 5.5,
            stealth_factor: (1.0 - savior as f32 * 0.025).max(0.55),
            cloak_duration: 1.35 + savior as f32 * 0.08,
            pulse_range: 72.0 + villain as f32 * 10.0,
            pulse_damage: 1 + (villain / 5) as i32,
            ability_cooldown: (2.6 - villain.max(savior) as f32 * 0.035).max(1.35),
        }
    }

    pub fn dominant_route(self) -> MoralChoice {
        if self.savior >= self.villain {
            MoralChoice::Savior
        } else {
            MoralChoice::Villain
        }
    }
}

pub fn campaign_route(choices: &[MoralChoice]) -> CampaignRoute {
    let profile = UpgradeProfile::from_choices(choices);
    if profile.savior >= profile.villain {
        CampaignRoute::Hero
    } else {
        CampaignRoute::Gremlin
    }
}
