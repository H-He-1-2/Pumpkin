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
    Grant,
    Revoke,
}

struct AdvancementCommandExecutor(AdvancementAction);

impl CommandExecutor for AdvancementCommandExecutor {
    fn execute<'a>(&'a self, context: &'a CommandContext) -> CommandExecutorResult<'a> {
        Box::pin(async move {
            let target = context.required_argument::<Arc<Player>>("player")?;
            let advancement = context.required_argument::<String>("advancement")?;

            match self.0 {
                AdvancementAction::Grant => {
                    target
                        .send_message(&TextComponent::text(format!(
                            "Advancement granted: {}",
                            advancement
                        )))
                        .await;
                }
                AdvancementAction::Revoke => {
                    target
                        .send_message(&TextComponent::text(format!(
                            "Advancement revoked: {}",
                            advancement
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

    let grant = literal("grant")
        .requires(grant_perm)
        .then(
            literal("target")
                .then(literal("advancement").then(
                    literal("*").executes(AdvancementCommandExecutor(AdvancementAction::Grant)),
                ))
                .executes(AdvancementCommandExecutor(AdvancementAction::Grant)),
        )
        .build();
    let revoke = literal("revoke");

    dispatcher.register(grant);
    dispatcher.register(revoke);
}
