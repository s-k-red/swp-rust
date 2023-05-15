use super::bot::Bot;

pub fn next_generation(last_gen: &Vec<Bot>) -> Vec<Bot> {
    let mut new_gen = Vec::new();

    for bot in last_gen {
        let mut b = bot.clone();
        b.mutate();
        new_gen.push(b); //crossover in the future?
    }

    new_gen
}
