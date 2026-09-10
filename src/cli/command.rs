use crate::CommandContext;
use adar::prelude::*;

#[TraitRef]
pub trait Command {
    fn execute(&self, context: CommandContext);
}
