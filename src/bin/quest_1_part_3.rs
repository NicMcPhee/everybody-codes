use itertools::Itertools;

fn main() {
    let creatures = include_str!("../inputs/quest_1_part_3.txt");
    let result = num_potions(creatures);
    println!("We need {result} potions.");
}

fn num_potions(creatures: &str) -> u32 {
    creatures
        .chars()
        .chunks(3)
        .into_iter()
        .map(potions_for_group)
        .sum()
}

fn potions_for_group(group: impl Iterator<Item = char>) -> u32 {
    let mut num_creatures: u32 = 0;
    let mut num_potions = 0;
    for c in group {
        if c != 'x' {
            num_creatures += 1;
            num_potions += potions_for_creature(c);
        }
    }
    num_potions + num_creatures * (num_creatures.saturating_sub(1))
}

fn potions_for_creature(creature: char) -> u32 {
    match creature {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        c => panic!("Should never see character '{c}'!"),
    }
}

#[cfg(test)]
mod tests {
    use super::num_potions;

    #[test]
    fn example() {
        let creatures = "xBxAAABCDxCC";
        let result = num_potions(creatures);
        assert_eq!(result, 30);
    }
}
