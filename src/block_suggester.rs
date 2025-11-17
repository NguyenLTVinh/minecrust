use crate::block::BlockType;
use strsim::jaro_winkler;
use strum::IntoEnumIterator;

pub struct BlockSuggestion {
    pub block_name: String,
    pub score: f64,
}

pub struct BlockSuggester;

impl BlockSuggester {
    pub fn suggest(input: &str, limit: usize) -> Vec<BlockSuggestion> {
        if input.is_empty() || input.to_lowercase() == "use" {
            return BlockType::iter()
                .take(limit)
                .map(|block_type| {
                    let block_name: &str = block_type.into();
                    BlockSuggestion {
                        block_name: block_name.to_string(),
                        score: 1.0,
                    }
                })
                .collect();
        }

        let input_lower = input.to_lowercase();

        let mut suggestions: Vec<BlockSuggestion> = BlockType::iter()
            .map(|block_type| {
                let block_name: &str = block_type.into();
                let name_lower = block_name.to_lowercase();

                let score = if name_lower == input_lower {
                    2.0
                } else if name_lower.starts_with(&input_lower) {
                    1.5 + (1.0 - (input_lower.len() as f64 / name_lower.len() as f64)) * 0.5
                } else {
                    jaro_winkler(&input_lower, &name_lower)
                };

                BlockSuggestion {
                    block_name: block_name.to_string(),
                    score,
                }
            })
            .filter(|s| s.score > 0.3)
            .collect();

        suggestions.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(limit);
        suggestions
    }
}
