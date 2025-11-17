use crate::block::BlockType;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum CommandResult {
    Success(String),
    Error(String),
    TimeChange(TimeChange),
}

#[derive(Debug, Clone)]
pub enum TimeChange {
    SetTime(f32),
    ToggleCycle,
}

pub struct CommandHandler;

impl CommandHandler {
    pub fn execute(input: &str) -> CommandResult {
        let input = input.trim();

        let mut words = input.split_whitespace();
        let first_word = match words.next() {
            Some(word) => word,
            None => return CommandResult::Error("Empty command".to_string()),
        };

        match first_word {
            "use" => Self::handle_use_command(input),
            "time" => Self::handle_time_command(input),
            _ => CommandResult::Error(format!("Unknown command: {}", first_word)),
        }
    }

    fn handle_use_command(input: &str) -> CommandResult {
        if !input.starts_with("use ") {
            return CommandResult::Error(
                "Invalid syntax. Expected: use <BlockTypeName>;".to_string(),
            );
        }

        if !input.ends_with(";") {
            return CommandResult::Error("Invalid syntax. Command must end with ';'".to_string());
        }

        let block_name = input
            .strip_prefix("use ")
            .unwrap()
            .strip_suffix(";")
            .unwrap()
            .trim();

        match BlockType::from_str(block_name) {
            Ok(block_type) => {
                CommandResult::Success(format!("Block type set to: {:?}", block_type))
            }
            Err(_) => CommandResult::Error(format!("Unknown block type: {}", block_name)),
        }
    }

    fn handle_time_command(input: &str) -> CommandResult {
        if !input.starts_with("time ") {
            return CommandResult::Error(
                "Invalid syntax. Expected: time <night|noon|dawn|dusk|toggle>;".to_string(),
            );
        }

        if !input.ends_with(";") {
            return CommandResult::Error("Invalid syntax. Command must end with ';'".to_string());
        }

        let arg = input
            .strip_prefix("time ")
            .unwrap()
            .strip_suffix(";")
            .unwrap()
            .trim();

        match arg {
            "night" => CommandResult::TimeChange(TimeChange::SetTime(0.0)),
            "noon" => CommandResult::TimeChange(TimeChange::SetTime(0.5)),
            "dawn" => CommandResult::TimeChange(TimeChange::SetTime(0.25)),
            "dusk" => CommandResult::TimeChange(TimeChange::SetTime(0.75)),
            "toggle" => CommandResult::TimeChange(TimeChange::ToggleCycle),
            _ => CommandResult::Error(format!("Unknown time argument: {}", arg)),
        }
    }
}
