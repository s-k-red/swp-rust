use rand::Rng;
use crate::training::bot::Bot;

pub fn select_parents(canidates: &[&Bot]) -> Vec<Bot>{
    let mut parents = Vec::new();

    parents.push(roulette_selection(canidates));
    parents.push(roulette_selection(canidates));

    parents
}

pub fn roulette_selection(canidates: &[&Bot]) -> Bot{
    let mut rnd = rand::thread_rng();
    let mut index = 0;
    let mut r = rnd.gen::<f32>(); //TODO: DOES NOT INCLUDE 1, problem?

    while r > 0.0 {
        r -= canidates[index].normalized_fitness;
        index += 1;
    }

    index -= 1;

    canidates[index].clone()
}
