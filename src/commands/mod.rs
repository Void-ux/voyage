use crate::structs::Command;

mod main;
mod help;
mod stats;

pub fn commands() -> Vec<Command> {
    main::commands()
        .into_iter()
        .chain(stats::commands())
        .chain(help::commands())
        .collect()
}
