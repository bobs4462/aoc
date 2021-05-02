/// --- Day 15: Science for Hungry People ---
pub struct D15;

use crate::solver::{Solution, Solver};

impl Solver for D15 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            let components: Vec<&[u8]> = l.split(|&c| c == b':').collect();
            if components.len() != 2 || components[1].split(|&c| c == b',').count() != 5 {
                return Err(format!(
                    "Invalid line:\n{}",
                    std::str::from_utf8(l).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let ingredients = D15::ingredients(data);

        Solution::new("", format!(""))
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        Solution::new("", format!(""))
    }
}

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(ingredient: &[u8]) -> Ingredient {
        let mut parts = ingredient.split(|&c| c == b':');
        let name = std::str::from_utf8(parts.next().unwrap())
            .unwrap()
            .to_string();
        let mut properties = parts.next().unwrap().split(|&c| c == b',');
        let mut capacity = properties.next().unwrap().split(|&c| c == b' ');
        let capacity = std::str::from_utf8(capacity.next().unwrap())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut durability = properties.next().unwrap().split(|&c| c == b' ');
        let durability = std::str::from_utf8(durability.next().unwrap())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut flavor = properties.next().unwrap().split(|&c| c == b' ');
        let flavor = std::str::from_utf8(flavor.next().unwrap())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut texture = properties.next().unwrap().split(|&c| c == b' ');
        let texture = std::str::from_utf8(texture.next().unwrap())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut calories = properties.next().unwrap().split(|&c| c == b' ');
        let calories = std::str::from_utf8(calories.next().unwrap())
            .unwrap()
            .parse::<i32>()
            .unwrap();

        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

impl D15 {
    fn ingredients(recipe: Vec<u8>) -> Vec<Ingredient> {
        let mut list = Vec::with_capacity(4);
        for l in recipe.split(|&c| c == b'\n') {
            list.push(Ingredient::new(l));
        }
        list
    }
}
