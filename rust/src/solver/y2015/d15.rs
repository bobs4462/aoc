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
        let mut distributor = Distributor::new();
        let mut best: i64 = i64::MIN;
        while let Some(distribution) = distributor.next() {
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            for (d, i) in distribution.iter().zip(ingredients.iter()) {
                capacity += d * i.capacity;
                durability += d * i.durability;
                flavor += d * i.flavor;
                texture += d * i.texture;
            }
            if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                continue;
            }
            best = best.max(capacity * durability * flavor * texture)
        }
        Solution::new("The best cookie has a score", best.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let ingredients = D15::ingredients(data);
        let mut distributor = Distributor::new();
        let mut best: i64 = i64::MIN;
        while let Some(distribution) = distributor.next() {
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;
            for (d, i) in distribution.iter().zip(ingredients.iter()) {
                capacity += d * i.capacity;
                durability += d * i.durability;
                flavor += d * i.flavor;
                texture += d * i.texture;
                calories += d * i.calories;
            }
            if calories != 500 {
                continue;
            }
            if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                continue;
            }
            best = best.max(capacity * durability * flavor * texture)
        }
        Solution::new(
            "The best cookie with 500 calories has a score",
            best.to_string(),
        )
    }
}

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn new(ingredient: &[u8]) -> Ingredient {
        let mut parts = ingredient.split(|&c| c == b':');
        let mut properties = parts.nth(1).unwrap().split(|&c| c == b',');
        let mut capacity = properties.next().unwrap().split(|&c| c == b' ');
        let capacity = std::str::from_utf8(capacity.nth(2).unwrap())
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut durability = properties.next().unwrap().split(|&c| c == b' ');
        let durability = std::str::from_utf8(durability.nth(2).unwrap())
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut flavor = properties.next().unwrap().split(|&c| c == b' ');
        let flavor = std::str::from_utf8(flavor.nth(2).unwrap())
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut texture = properties.next().unwrap().split(|&c| c == b' ');
        let texture = std::str::from_utf8(texture.nth(2).unwrap())
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut calories = properties.next().unwrap().split(|&c| c == b' ');
        let calories = std::str::from_utf8(calories.nth(2).unwrap())
            .unwrap()
            .parse::<i64>()
            .unwrap();

        Ingredient {
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

struct Distributor {
    state: [i64; 4],
}

impl Distributor {
    fn new() -> Self {
        Distributor {
            state: [1, 1, 0, 0],
        }
    }

    fn next(&mut self) -> Option<&[i64]> {
        if self.done() {
            return None;
        }
        if self.state[2] == self.cap(2) {
            if self.state[1] == self.cap(1) {
                self.state[0] += 1;
                self.state[1] = 1;
            } else {
                self.state[1] += 1;
            }
            self.state[2] = 1;
        } else {
            self.state[2] += 1;
        }
        self.state[3] = self.cap(3);
        Some(&self.state)
    }

    #[inline]
    fn done(&self) -> bool {
        let first = unsafe { self.state.get_unchecked(0) };
        *first == 97
    }

    #[inline]
    fn cap(&self, index: usize) -> i64 {
        let before = self.state[0..index].iter().fold(0, |acc, v| acc + v);
        100 - before - (3 - index) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::Distributor;

    #[test]
    fn test_distributor() {
        let mut dist = Distributor::new();
        assert_eq!(dist.next().unwrap(), &[1, 1, 1, 97]);
        for _ in 0..95 {
            dist.next();
        }
        assert_eq!(dist.next().unwrap(), &[1, 1, 97, 1]);
        assert_eq!(dist.next().unwrap(), &[1, 2, 1, 96]);
    }
}
