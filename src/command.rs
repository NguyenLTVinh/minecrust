use crate::block::BlockType;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum CommandResult {
    Success(String),
    Error(String),
    TimeChange(TimeChange),
    RotationChange(Rotation),
}

#[derive(Debug, Clone)]
pub enum TimeChange {
    SetTime(f32),
    ToggleCycle,
}

pub struct CommandHandler;

#[derive(Debug, Clone)]
pub struct Rotation {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl CommandHandler {
    pub fn execute(input: &str) -> CommandResult {
        let input = input.trim();

        let mut words = input.split_whitespace();
        let first_word = match words.next() {
            Some(word) => word,
            None => return CommandResult::Error("Empty command".to_string()),
        };

        if !input.ends_with(";") {
            return CommandResult::Error("Invalid syntax. Command must end with ';'".to_string());
        }

        match first_word {
            "use" => Self::handle_use_command(input),
            "time" => Self::handle_time_command(input),
            "rotate" => Self::handle_rotate_command(input),
            _ => CommandResult::Error(format!("Unknown command: {}", first_word)),
        }
    }

    fn handle_use_command(input: &str) -> CommandResult {
        if !input.starts_with("use ") {
            return CommandResult::Error(
                "Invalid syntax. Expected: use <BlockTypeName>;".to_string(),
            );
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

    fn handle_rotate_command(input: &str) -> CommandResult {
        if !input.starts_with("rotate ") {
            return CommandResult::Error(
                "Invalid syntax. Expected: rotate <x> <y> <z>;".to_string(),
            );
        }

        let args_str = input
            .strip_prefix("rotate ")
            .unwrap()
            .strip_suffix(";")
            .unwrap()
            .trim();

        let parts: Vec<&str> = args_str.split_whitespace().collect();

        if parts.len() != 3 {
            return CommandResult::Error(
                "Invalid syntax. Expected: rotate <x> <y> <z>;".to_string(),
            );
        }

        let parse_and_validate = |s: &str, axis_name: &str| -> Result<u16, CommandResult> {
            match s.parse::<u16>() {
                Ok(val) => {
                    if val % 90 != 0 {
                        Err(CommandResult::Error(format!(
                            "Invalid {} value. Must be a multiple of 90.",
                            axis_name
                        )))
                    } else {
                        Ok(val)
                    }
                }
                Err(_) => Err(CommandResult::Error(format!(
                    "Invalid {} value. Must be a u16.",
                    axis_name
                ))),
            }
        };

        let x = match parse_and_validate(parts[0], "x") {
            Ok(val) => val,
            Err(e) => return e,
        };
        let y = match parse_and_validate(parts[1], "y") {
            Ok(val) => val,
            Err(e) => return e,
        };
        let z = match parse_and_validate(parts[2], "z") {
            Ok(val) => val,
            Err(e) => return e,
        };

        CommandResult::RotationChange(Rotation { x, y, z })
    }
}
