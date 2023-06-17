use crate::structs::{Command, CommandError, Context};

enum HelpCommandMode<'a> {
    Root,
    Group(&'a Command),
    Command(&'a Command)
}

fn format_params<U, E>(command: &poise::Command<U, E>) -> String {
    command.parameters.iter().map(|p| {
        if p.required {
            format!("<{}> ", p.name)
        } else {
            format!("[{}] ", p.name)
        }
    }).collect()
}

fn find_command<'a, U, E>(
    commands: &'a [poise::Command<U, E>],
    remaining_message: &'a str,
    case_insensitive: bool,
) -> Option<(&'a poise::Command<U, E>, &'a str, &'a str)>
where
    U: Send + Sync,
{
    let string_equal = if case_insensitive {
        |a: &str, b: &str| a.eq_ignore_ascii_case(b)
    } else {
        |a: &str, b: &str| a == b
    };

    let (command_name, remaining_message) = {
        let mut iter = remaining_message.splitn(2, char::is_whitespace);
        (iter.next().unwrap(), iter.next().unwrap_or("").trim_start())
    };

    for command in commands {
        let primary_name_matches = string_equal(&command.name, command_name);
        let alias_matches = command
            .aliases
            .iter()
            .any(|alias| string_equal(alias, command_name));
        if !primary_name_matches && !alias_matches {
            continue;
        }

        return Some(
            find_command(&command.subcommands, remaining_message, case_insensitive).unwrap_or((
                command,
                command_name,
                remaining_message,
            )),
        );
    }

    None
}

/// Displays a list of my commands
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), CommandError> {
    let framework_options = ctx.framework().options();
    let commands_ = &framework_options.commands;

    let mode = match command.as_deref() {
        None => HelpCommandMode::Root,
        Some(command) => {
            let mut subcommand_iterator = command.split(' ');
            let top_level_command = subcommand_iterator.next().unwrap();

            let match_ = find_command(commands_, top_level_command, false);
            if match_.is_none() {
                ctx.say(format!("Command {} not found", command)).await?;
                return Ok(())
            }

            let (command_obj, _, _) = match_.unwrap();
            let remaining_args: Vec<_> = subcommand_iterator.collect();
            if !command_obj.subcommands.is_empty() && remaining_args.is_empty() {
                HelpCommandMode::Group(command_obj)
            } else {
                HelpCommandMode::Command(command_obj)
            }
        }
    };

    ctx.send(|b| b.embed(|e| e
        .title(match &mode {
            HelpCommandMode::Root => ctx.serenity_context().cache.current_user_field(|user| user.name.clone()) + " Help",
            HelpCommandMode::Command(c) | HelpCommandMode::Group(c) => format!("{} {}", c.qualified_name.clone(), &*format_params(c))
        })
        .description(match &mode {
            HelpCommandMode::Root => {
                commands_.iter().map(|c| {
                    format!("**{} {}**\n{}", c.qualified_name.clone(), &*format_params(c), c.description.clone().unwrap_or("No description provided".to_owned()))
                }).collect::<Vec<String>>().join("\n\n")
            },
            HelpCommandMode::Command(c) | HelpCommandMode::Group(c) => c.description.clone().unwrap_or("No description provided".to_owned())
        })
        .colour(0xC0C0C0)
    )).await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [help()]
}
