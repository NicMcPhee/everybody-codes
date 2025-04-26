fn main() {
    let creatures = include_str!("../inputs/quest_1_part_1.txt");
    let result = num_potions(creatures);
    println!("We need {result} potions.");
}

fn num_potions(creatures: &str) -> u32 {
    creatures.chars().map(potions_for_creature).sum()
}

fn potions_for_creature(creature: char) -> u32 {
    match creature {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        c => panic!("Should never see character '{c}'!"),
    }
}

#[cfg(test)]
mod tests {
    use super::num_potions;

    #[test]
    fn example() {
        let creatures = "ABBAC";
        let result = num_potions(creatures);
        assert_eq!(result, 5);
    }
}
