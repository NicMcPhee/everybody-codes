use std::{collections::HashSet, ops::Not, str::FromStr};

use aho_corasick::AhoCorasick;

fn main() {
    let input = include_str!("../inputs/quest_2_part_2.txt");
    let challenge: Challenge = input.parse().unwrap();
    let num_runes = challenge.count_runic_symbols();
    println!("The number of runic symbols in the text that matched was {num_runes}.");
}

#[derive(Debug)]
pub enum ChallengeParseError {
    MissingWordLine,
    MissingPrefixInWords,
    MissingMiddleLine,
    MiddleLineNotBlank,
    MissingTextLine,
}

#[derive(Debug)]
pub struct Challenge {
    words: Vec<String>,
    text: String,
}

impl Challenge {
    pub fn count_runic_symbols(self) -> usize {
        let mut matched_positions: HashSet<usize> = HashSet::new();
        let all_words = self
            .words
            .into_iter()
            .flat_map(|w| [w.chars().rev().collect::<String>(), w])
            .collect::<Vec<_>>();
        let matcher = AhoCorasick::new(&all_words).unwrap();
        for r#match in matcher.find_overlapping_iter(&self.text) {
            for position in r#match.range() {
                matched_positions.insert(position);
            }
        }
        matched_positions.len()
    }
}

impl FromStr for Challenge {
    type Err = ChallengeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, '\n');
        let words = parts
            .next()
            .ok_or(ChallengeParseError::MissingWordLine)?
            .strip_prefix("WORDS:")
            .ok_or(ChallengeParseError::MissingPrefixInWords)?
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let blank_line = parts.next().ok_or(ChallengeParseError::MissingMiddleLine)?;
        if blank_line.is_empty().not() {
            return Err(ChallengeParseError::MiddleLineNotBlank);
        }
        let text = parts
            .next()
            .ok_or(ChallengeParseError::MissingTextLine)?
            .to_string();
        Ok(Challenge { words, text })
    }
}

#[cfg(test)]
mod tests {
    use aho_corasick::AhoCorasick;

    use super::Challenge;

    #[test]
    fn successful_parsing() {
        let input = "WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ";
        let Challenge { words, text } = input.parse().unwrap();
        assert_eq!(words, vec!["THE", "OWE", "MES", "ROD", "HER", "QAQ"]);
        assert_eq!(
            text,
            "AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ"
        );
    }

    #[test]
    fn example() {
        let input = "WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ";
        let challenge: Challenge = input.parse().unwrap();
        let num_runes = challenge.count_runic_symbols();
        assert_eq!(num_runes, 42);
    }

    #[test]
    fn repeated_pattern() {
        let words = ["I", "I"];
        let text = "I am Legend";
        let matcher = AhoCorasick::new(words).unwrap();
        let matches = matcher.find_overlapping_iter(text).collect::<Vec<_>>();
        assert_eq!(matches.len(), 2);
    }
}
