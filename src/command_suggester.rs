use crate::block_suggester::BlockSuggester;
use strsim::jaro_winkler;

#[derive(Clone, Debug)]
pub enum CommandSuggestion {
    Command(String, f64),
    Argument(String, String, f64),
    Block(String, String, f64),
}

impl CommandSuggestion {
    pub fn text(&self) -> String {
        match self {
            CommandSuggestion::Command(cmd, _) => cmd.clone(),
            CommandSuggestion::Argument(_, arg, _) => arg.clone(),
            CommandSuggestion::Block(_, block, _) => block.clone(),
        }
    }

    pub fn score(&self) -> f64 {
        match self {
            CommandSuggestion::Command(_, score) => *score,
            CommandSuggestion::Argument(_, _, score) => *score,
            CommandSuggestion::Block(_, _, score) => *score,
        }
    }
}

pub struct CommandSuggester;

impl CommandSuggester {
    fn get_available_commands() -> Vec<&'static str> {
        vec!["use", "time", "rorate"]
    }

    fn get_available_arguments(command: &str) -> Vec<&'static str> {
        match command {
            "use" => vec![],
            "time" => vec!["night", "noon", "dawn", "dusk", "toggle"],
            _ => vec![],
        }
    }

    fn get_block_suggestions(input: &str, limit: usize) -> Vec<CommandSuggestion> {
        BlockSuggester::suggest(input, limit)
            .into_iter()
            .map(|bs| CommandSuggestion::Block("use".to_string(), bs.block_name, bs.score))
            .collect()
    }

    fn suggest_commands(input: &str, limit: usize) -> Vec<CommandSuggestion> {
        if input.is_empty() {
            return Self::get_available_commands()
                .into_iter()
                .take(limit)
                .map(|cmd| CommandSuggestion::Command(cmd.to_string(), 1.0))
                .collect();
        }

        let input_lower = input.to_lowercase();
        let mut suggestions: Vec<CommandSuggestion> = Self::get_available_commands()
            .into_iter()
            .map(|cmd| {
                let score = if cmd == input_lower {
                    2.0
                } else if cmd.starts_with(&input_lower) {
                    1.5 + (1.0 - (input_lower.len() as f64 / cmd.len() as f64)) * 0.5
                } else {
                    jaro_winkler(&input_lower, cmd)
                };
                CommandSuggestion::Command(cmd.to_string(), score)
            })
            .filter(|s| s.score() > 0.3)
            .collect();

        suggestions.sort_by(|a, b| {
            b.score()
                .partial_cmp(&a.score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(limit);
        suggestions
    }

    fn suggest_arguments(command: &str, input: &str, limit: usize) -> Vec<CommandSuggestion> {
        let available_args = Self::get_available_arguments(command);

        if available_args.is_empty() {
            return vec![];
        }

        if input.is_empty() {
            return available_args
                .into_iter()
                .take(limit)
                .map(|arg| CommandSuggestion::Argument(command.to_string(), arg.to_string(), 1.0))
                .collect();
        }

        let input_lower = input.to_lowercase();
        let mut suggestions: Vec<CommandSuggestion> = available_args
            .into_iter()
            .map(|arg| {
                let score = if arg == input_lower {
                    2.0
                } else if arg.starts_with(&input_lower) {
                    1.5 + (1.0 - (input_lower.len() as f64 / arg.len() as f64)) * 0.5
                } else {
                    jaro_winkler(&input_lower, arg)
                };
                CommandSuggestion::Argument(command.to_string(), arg.to_string(), score)
            })
            .filter(|s| s.score() > 0.3)
            .collect();

        suggestions.sort_by(|a, b| {
            b.score()
                .partial_cmp(&a.score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(limit);
        suggestions
    }

    pub fn suggest(input: &str, limit: usize) -> Vec<CommandSuggestion> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.len() {
            0 => Self::suggest_commands("", limit),
            1 => {
                let word = parts[0];
                if word == "use" {
                    Self::get_block_suggestions("", limit)
                } else if Self::get_available_commands().contains(&word) {
                    Self::suggest_arguments(word, "", limit)
                } else {
                    Self::suggest_commands(word, limit)
                }
            }
            _ => {
                let command = parts[0];
                let arg_input = parts[1..].join(" ");
                if command == "use" {
                    Self::get_block_suggestions(&arg_input, limit)
                } else if Self::get_available_commands().contains(&command) {
                    Self::suggest_arguments(command, &arg_input, limit)
                } else {
                    vec![]
                }
            }
        }
    }
}
