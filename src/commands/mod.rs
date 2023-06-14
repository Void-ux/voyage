use crate::structs::Command;

mod main;
mod stats;

pub fn commands() -> Vec<Command> {
    main::commands()
        .into_iter()
        .chain(stats::commands())
        .collect()
}
