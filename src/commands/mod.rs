use crate::structs::Command;

mod main;
mod help;
mod stats;
mod economy;

pub fn commands() -> Vec<Command> {
    main::commands()
        .into_iter()
        .chain(stats::commands())
        .chain(help::commands())
        .chain(economy::commands())
        .collect()
}
