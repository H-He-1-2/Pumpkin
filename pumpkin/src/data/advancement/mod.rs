use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub mod vanilla;

pub struct AdvancementManager {
    advancements: RwLock<HashMap<String, PlayerAdvancementState>>,
    pub vanilla: vanilla::VanillaAdvancements,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerAdvancementState {
    pub completed: HashMap<String, CompletedAdvancement>,
    pub criteria: HashMap<String, CriterionProgress>,
    pub selected_tab: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CompletedAdvancement {
    pub id: String,
    pub criterion: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CriterionProgress {
    pub criterion_id: String,
    pub achieved: bool,
    pub date: Option<i64>,
}

impl AdvancementManager {
    pub fn new() -> Self {
        Self {
            advancements: RwLock::new(HashMap::new()),
            vanilla: vanilla::VanillaAdvancements::new(),
        }
    }

    pub async fn get_player_state(&self, player_id: &str) -> Option<PlayerAdvancementState> {
        let advancements = self.advancements.read().await;
        advancements.get(player_id).cloned()
    }

    pub async fn init_player(&self, player_id: String) {
        let mut advancements = self.advancements.write().await;
        if !advancements.contains_key(&player_id) {
            advancements.insert(player_id, PlayerAdvancementState::default());
        }
    }

    pub async fn grant_criterion(
        &self,
        player_id: &str,
        advancement_id: &str,
        criterion_id: &str,
    ) -> bool {
        let mut advancements = self.advancements.write().await;
        if let Some(state) = advancements.get_mut(player_id) {
            let criterion_key = format!("{}:{}", advancement_id, criterion_id);
            if !state.criteria.contains_key(&criterion_key) {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_millis() as i64);

                state.criteria.insert(
                    criterion_key,
                    CriterionProgress {
                        criterion_id: criterion_id.to_string(),
                        achieved: true,
                        date: now,
                    },
                );
                return true;
            }
        }
        false
    }

    pub async fn complete_advancement(
        &self,
        player_id: &str,
        advancement_id: &str,
        criterion_id: &str,
    ) -> bool {
        let mut advancements = self.advancements.write().await;
        if let Some(state) = advancements.get_mut(player_id) {
            if !state.completed.contains_key(advancement_id) {
                state.completed.insert(
                    advancement_id.to_string(),
                    CompletedAdvancement {
                        id: advancement_id.to_string(),
                        criterion: criterion_id.to_string(),
                    },
                );
                return true;
            }
        }
        false
    }

    pub async fn set_selected_tab(&self, player_id: &str, tab_id: Option<String>) {
        let mut advancements = self.advancements.write().await;
        if let Some(state) = advancements.get_mut(player_id) {
            state.selected_tab = tab_id;
        }
    }
}

impl Default for AdvancementManager {
    fn default() -> Self {
        Self {
            advancements: RwLock::new(HashMap::new()),
            vanilla: vanilla::VanillaAdvancements::default(),
        }
    }
}
