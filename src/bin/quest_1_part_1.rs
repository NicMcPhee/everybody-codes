fn main() {
    println!("Hello world");
}

fn num_potions(creatures: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::num_potions;

    #[test]
    fn example() {
        let creatures = "ABBAC".to_string();
        let result = num_potions(creatures);
        assert_eq!(result, 5);
    }
}
