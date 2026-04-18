use std::sync::Arc;

use pumpkin_util::{
    permission::{Permission, PermissionDefault, PermissionRegistry},
    text::TextComponent,
};

use crate::{
    command::{
        argument_builder::{ArgumentBuilder, literal},
        context::command_context::CommandContext,
        node::{CommandExecutor, CommandExecutorResult},
    },
    entity::player::Player,
};

const DESCRIPTION: &str = "Grant or revoke player advancements.";
const PERMISSION: &str = "minecraft:command.advancement";

enum AdvancementAction {
    GrantAll,
    GrantAdvancement,
    GrantCriterion,
    RevokeAll,
    RevokeAdvancement,
    RevokeCriterion,
}

struct AdvancementCommandExecutor(AdvancementAction);

impl CommandExecutor for AdvancementCommandExecutor {
    fn execute<'a>(&'a self, context: &'a CommandContext) -> CommandExecutorResult<'a> {
        Box::pin(async move {
            let target = context.required_argument::<Arc<Player>>("player")?;
            let server = target.world().server.upgrade().unwrap();
            
            match self.0 {
                AdvancementAction::GrantAll => {
                    // Grant all advancements to the player
                    let advancements = &server.data.advancement_manager.vanilla.advancements;
                    let mut granted_count = 0;
                    
                    for adv in advancements {
                        for criterion in &adv.criteria {
                            target.grant_advancement(&adv.id, criterion).await;
                        }
                        granted_count += 1;
                    }
                    
                    target
                        .send_message(&TextComponent::text(format!(
                            "Granted {} advancements to {}",
                            granted_count,
                            target.gameprofile.name
                        )))
                        .await;
                }
                AdvancementAction::GrantAdvancement => {
                    // Grant specific advancement with all its criteria
                    let advancement = context.required_argument::<String>("advancement")?;
                    
                    // Get advancement data to find all criteria
                    if let Some(adv_data) = server
                        .data
                        .advancement_manager
                        .vanilla
                        .advancement_map
                        .get(&advancement)
                    {
                        // Grant all criteria for this advancement
                        for criterion in &adv_data.criteria {
                            target.grant_advancement(&advancement, criterion).await;
                        }
                        
                        target
                            .send_message(&TextComponent::text(format!(
                                "Granted advancement {} to {}",
                                advancement,
                                target.gameprofile.name
                            )))
                            .await;
                    } else {
                        target
                            .send_message(&TextComponent::text(format!(
                                "Unknown advancement: {}",
                                advancement
                            )))
                            .await;
                    }
                }
                AdvancementAction::GrantCriterion => {
                    // Grant specific criterion of an advancement
                    let advancement = context.required_argument::<String>("advancement")?;
                    let criterion = context.required_argument::<String>("criterion")?;
                    
                    // Verify advancement exists
                    if server
                        .data
                        .advancement_manager
                        .vanilla
                        .advancement_map
                        .contains_key(&advancement)
                    {
                        target.grant_advancement(&advancement, &criterion).await;
                        target
                            .send_message(&TextComponent::text(format!(
                                "Granted criterion {} of advancement {} to {}",
                                criterion,
                                advancement,
                                target.gameprofile.name
                            )))
                            .await;
                    } else {
                        target
                            .send_message(&TextComponent::text(format!(
                                "Unknown advancement: {}",
                                advancement
                            )))
                            .await;
                    }
                }
                AdvancementAction::RevokeAll => {
                    target
                        .send_message(&TextComponent::text(format!(
                            "Revoking all advancements from {} (not yet implemented)",
                            target.gameprofile.name
                        )))
                        .await;
                }
                AdvancementAction::RevokeAdvancement => {
                    let advancement = context.required_argument::<String>("advancement")?;
                    target
                        .send_message(&TextComponent::text(format!(
                            "Revoking advancement {} from {} (not yet implemented)",
                            advancement,
                            target.gameprofile.name
                        )))
                        .await;
                }
                AdvancementAction::RevokeCriterion => {
                    let advancement = context.required_argument::<String>("advancement")?;
                    let criterion = context.required_argument::<String>("criterion")?;
                    target
                        .send_message(&TextComponent::text(format!(
                            "Revoking criterion {} of advancement {} from {} (not yet implemented)",
                            criterion,
                            advancement,
                            target.gameprofile.name
                        )))
                        .await;
                }
            }

            Ok(())
        })
    }
}

pub fn register(dispatcher: &mut crate::command::node::dispatcher::CommandDispatcher) {
    let grant_perm = PermissionRegistry::default()
        .add(PERMISSION, PermissionDefault::Op)
        .build();

    // Grant command: /advancement grant <player> [advancement]
    let grant = literal("grant")
        .requires(grant_perm.clone())
        .then(
            literal("target")
                .argument("player", crate::command::argument_types::entity_selector::EntitySelector::new())
                .then(
                    literal("everything")
                        .executes(AdvancementCommandExecutor(AdvancementAction::GrantAll)),
                )
                .then(
                    literal("from")
                        .then(
                            literal("advancement")
                                .argument("advancement", crate::command::argument_types::resource_location::ResourceLocation::new())
                                .then(
                                    literal("only")
                                        .argument("criterion", crate::command::argument_types::string::String::new())
                                        .executes(AdvancementCommandExecutor(AdvancementAction::GrantCriterion)),
                                )
                                .executes(AdvancementCommandExecutor(AdvancementAction::GrantAdvancement)),
                        ),
                ),
        )
        .build();
    
    // Revoke command: /advancement revoke <player> [advancement]
    let revoke = literal("revoke")
        .requires(grant_perm)
        .then(
            literal("target")
                .argument("player", crate::command::argument_types::entity_selector::EntitySelector::new())
                .then(
                    literal("everything")
                        .executes(AdvancementCommandExecutor(AdvancementAction::RevokeAll)),
                )
                .then(
                    literal("from")
                        .then(
                            literal("advancement")
                                .argument("advancement", crate::command::argument_types::resource_location::ResourceLocation::new())
                                .then(
                                    literal("only")
                                        .argument("criterion", crate::command::argument_types::string::String::new())
                                        .executes(AdvancementCommandExecutor(AdvancementAction::RevokeCriterion)),
                                )
                                .executes(AdvancementCommandExecutor(AdvancementAction::RevokeAdvancement)),
                        ),
                ),
        )
        .build();

    dispatcher.register(grant);
    dispatcher.register(revoke);
}
