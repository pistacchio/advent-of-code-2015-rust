use std::fs::read_to_string;
use std::ops::{Mul, Add};
use std::cmp::max;

const INPUT_FILE: &str = "data/day_15.txt";

#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn from_str(s: &str) -> Self {
        let words: Vec<_> = s.split_whitespace().collect();
        Self {
            name: words[0].trim_end_matches('.').to_owned(),
            capacity: words[2].trim_end_matches(',').parse::<i64>().unwrap(),
            durability: words[4].trim_end_matches(',').parse::<i64>().unwrap(),
            flavor: words[6].trim_end_matches(',').parse::<i64>().unwrap(),
            texture: words[8].trim_end_matches(',').parse::<i64>().unwrap(),
            calories: words[10].trim_end_matches(',').parse::<i64>().unwrap(),
        }
    }

    fn score(&self) -> i64 {
        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Mul<i64> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Self {
            name: self.name.clone(),
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            name: self.name.clone(),
            capacity: max(self.capacity + rhs.capacity, 0),
            durability: max(self.durability + rhs.durability, 0),
            flavor: max(self.flavor + rhs.flavor, 0),
            texture: max(self.texture + rhs.texture, 0),
            calories: max(self.calories + rhs.calories, 0),
        }
    }
}

fn make_permutations(range: &[i64], sum: i64) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![];

    for a in range {
        for b in range {
            for c in range {
                result.push(vec![*a, *b, *c, sum - (a + b + c)]);
            }
        }
    }

    result
}

fn find_optimal_recipe(ingredients: &[Ingredient], exact_calories: Option<i64>) -> i64 {
    let combinations = make_permutations(&(1..100).collect::<Vec<i64>>(), 100);

    let mut best_score = 0;
    let mut best_score_exact_calories = 0;

    for combination in combinations.iter() {
        let combination_ingredients = combination.iter()
            .enumerate()
            .map(|(idx, perc)| (ingredients[idx].clone() * (*perc)))
            .collect::<Vec<_>>();

        let combination_sum: Ingredient = combination_ingredients.iter()
            .skip(1)
            .fold(combination_ingredients[0].clone(), |acc, i| {
                acc + i.to_owned()
            });

        // exact calories check
        if let Some(calories) = exact_calories {
            if combination_sum.calories == calories && combination_sum.score() > best_score_exact_calories {
                best_score_exact_calories = combination_sum.score();
            }
        }

        if combination_sum.score() > best_score {
            best_score = combination_sum.score();
        }
    }

    if exact_calories.is_none() { best_score } else { best_score_exact_calories }
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();

    let ingredients: Vec<_> = input.lines().map(Ingredient::from_str).collect();
    find_optimal_recipe(&ingredients, None).to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();

    let ingredients: Vec<_> = input.lines().map(Ingredient::from_str).collect();
    find_optimal_recipe(&ingredients, Some(500)).to_string()
}