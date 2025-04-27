use std::{ops::Not, str::FromStr};

use regex::Regex;

fn main() {
    let input = include_str!("../inputs/quest_2_part_1.txt");
    let challenge: Challenge = input.parse().unwrap();
    let num_runes = challenge.count_runes();
    println!("The number of rune occurrences was {num_runes}.");
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
    pub fn count_runes(&self) -> usize {
        let regex = Regex::new(&self.words.join("|")).unwrap();
        // THIS DOESN'T WORK. `find_iter()` returns _non-overlapping_ matches, so we
        // undercount.
        // TODO: Try the `needle` crate, which provides `boyer-moore`.
        // Thanks to @JustusFluegel for pointing out the existence of the `needle` crate.
        regex.find_iter(&self.text).count()
    }
}

impl FromStr for Challenge {
    type Err = ChallengeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let words = lines
            .next()
            .ok_or(ChallengeParseError::MissingWordLine)?
            .strip_prefix("WORDS:")
            .ok_or(ChallengeParseError::MissingPrefixInWords)?
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let blank_line = lines.next().ok_or(ChallengeParseError::MissingMiddleLine)?;
        if blank_line.is_empty().not() {
            return Err(ChallengeParseError::MiddleLineNotBlank);
        }
        let text = lines
            .next()
            .ok_or(ChallengeParseError::MissingTextLine)?
            .to_string();
        Ok(Challenge { words, text })
    }
}

#[cfg(test)]
mod tests {
    use super::Challenge;

    #[test]
    fn successful_parsing() {
        let input = "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
        let Challenge { words, text } = input.parse().unwrap();
        assert_eq!(words, vec!["THE", "OWE", "MES", "ROD", "HER"]);
        assert_eq!(text, "AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE");
    }

    #[test]
    fn example() {
        let input = "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
        let challenge: Challenge = input.parse().unwrap();
        let num_runes = challenge.count_runes();
        assert_eq!(num_runes, 4);
    }
}
