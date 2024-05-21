use anyhow::{anyhow, Result};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Map, Value};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Difficulty {
    pub num_items: usize,
    pub better_than_baseline: u32,
}

impl crate::DifficultyTrait<2> for Difficulty {
    fn from_arr(arr: &[i32; 2]) -> Self {
        Self {
            num_items: arr[0] as usize,
            better_than_baseline: arr[1] as u32,
        }
    }

    fn to_arr(&self) -> [i32; 2] {
        [self.num_items as i32, self.better_than_baseline as i32]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solution {
    pub items: Vec<usize>,
}

impl crate::SolutionTrait for Solution {}

impl TryFrom<Map<String, Value>> for Solution {
    type Error = serde_json::Error;

    fn try_from(v: Map<String, Value>) -> Result<Self, Self::Error> {
        from_value(Value::Object(v))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Challenge {
    pub seed: u32,
    pub difficulty: Difficulty,
    pub weights: Vec<u32>,
    pub values: Vec<u32>,
    pub max_weight: u32,
    pub min_value: u32,
}

impl crate::ChallengeTrait<Solution, Difficulty, 2> for Challenge {
    fn generate_instance(seed: u32, difficulty: &Difficulty) -> Result<Challenge> {
        let mut rng: StdRng = StdRng::seed_from_u64(seed as u64);

        let weights: Vec<u32> = (0..difficulty.num_items)
            .map(|_| rng.gen_range(1..50))
            .collect();
        let values: Vec<u32> = (0..difficulty.num_items)
            .map(|_| rng.gen_range(1..50))
            .collect();
        let max_weight: u32 = weights.iter().sum::<u32>() / 2;

        // Baseline greedy algorithm
        let mut sorted_value_to_weight_ratio: Vec<usize> = (0..difficulty.num_items).collect();
        sorted_value_to_weight_ratio.sort_by(|&a, &b| {
            let ratio_a = values[a] as f64 / weights[a] as f64;
            let ratio_b = values[b] as f64 / weights[b] as f64;
            ratio_b.partial_cmp(&ratio_a).unwrap()
        });

        let mut total_weight = 0;
        let mut min_value = 0;
        for &item in &sorted_value_to_weight_ratio {
            if total_weight + weights[item] > max_weight {
                continue;
            }
            min_value += values[item];
            total_weight += weights[item];
        }
        min_value = (min_value as f64 * (1.0 + difficulty.better_than_baseline as f64 / 1000.0))
            .round() as u32;

        Ok(Challenge {
            seed,
            difficulty: difficulty.clone(),
            weights,
            values,
            max_weight,
            min_value,
        })
    }

    fn verify_solution(&self, solution: &Solution) -> Result<()> {
        let selected_items: HashSet<usize> = solution.items.iter().cloned().collect();
        if selected_items.len() != solution.items.len() {
            return Err(anyhow!("Duplicate items selected."));
        }
        if let Some(item) = selected_items
            .iter()
            .find(|&&item| item >= self.weights.len())
        {
            return Err(anyhow!("Item ({}) is out of bounds", item));
        }

        let total_weight = selected_items
            .iter()
            .map(|&item| self.weights[item])
            .sum::<u32>();
        if total_weight > self.max_weight {
            return Err(anyhow!(
                "Total weight ({}) exceeded max weight ({})",
                total_weight,
                self.max_weight
            ));
        }
        let total_value = selected_items
            .iter()
            .map(|&item| self.values[item])
            .sum::<u32>();
        if total_value < self.min_value {
            Err(anyhow!(
                "Total value ({}) does not reach minimum value ({})",
                total_value,
                self.min_value
            ))
        } else {
            Ok(())
        }
    }
}
