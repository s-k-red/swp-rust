use itertools::{Itertools, Zip};
use rand::Rng;
use rulinalg::matrix::{BaseMatrix, BaseMatrixMut};
use crate::config::{HIDDEN_LAYERS, HIDDEN_NODES, INPUT_NODES, OUTPUT_NODES, PERCENTAGE_RAND_NEURONS_CROSSOVER};
use crate::training::bot::Bot;

pub fn crossover(parents: &[Bot]) -> Vec<Bot>{
    crossover_by_neuron(parents)
}

pub fn crossover_by_neuron(parents: &[Bot]) -> Vec<Bot>{
    let mut offspring = parents.iter().cloned().collect_vec();
    let (o1, o2) = offspring.split_at_mut(1);
    let max_layer = HIDDEN_LAYERS+1; // output + hidden
    let mut rnd = rand::thread_rng();

    let layer_to_crossover_neuron = rnd.gen_range(0..max_layer);

    let mut already_done_neurons = Vec::new();
    let mut neuron = 0;

    if layer_to_crossover_neuron == 0 {
        //input, does not include bias crossover because input has no pre calculated bias
        for _n in 0..((INPUT_NODES as f32 * PERCENTAGE_RAND_NEURONS_CROSSOVER) as usize){
            neuron = rnd.gen_range(0..INPUT_NODES);
            while already_done_neurons.contains(&neuron) {
                neuron = rnd.gen_range(0..INPUT_NODES);
            }
            let mut weights_a = o1[0].brain.weights[0].col_mut(neuron);
            let mut weights_b = o2[0].brain.weights[0].col_mut(neuron);

            for w in weights_a.iter_mut().zip(weights_b.iter_mut()) {
                std::mem::swap(w.0, w.1);
            }

            already_done_neurons.push(neuron);
        }
    } else if layer_to_crossover_neuron == max_layer {
        //output, no weights because weights are only from hidden -> output
        for _n in 0..((OUTPUT_NODES as f32 * PERCENTAGE_RAND_NEURONS_CROSSOVER) as usize){
            neuron = rnd.gen_range(0..OUTPUT_NODES);
            while already_done_neurons.contains(&neuron) {
                neuron = rnd.gen_range(0..OUTPUT_NODES);
            }
            let mut biases_a = o1[0].brain.biases[HIDDEN_LAYERS].col_mut(0);
            let mut biases_b = o2[0].brain.biases[HIDDEN_LAYERS].col_mut(0);

            for (i, w) in biases_a.iter_mut().zip(biases_b.iter_mut()).enumerate() {
                if i == neuron {
                    std::mem::swap(w.0, w.1);
                }
            }

            already_done_neurons.push(neuron);
        }
    } else {
        //hidden
        for _n in 0..((HIDDEN_NODES as f32 * PERCENTAGE_RAND_NEURONS_CROSSOVER) as usize){
            neuron = rnd.gen_range(0..HIDDEN_NODES);
            while already_done_neurons.contains(&neuron) {
                neuron = rnd.gen_range(0..HIDDEN_NODES);
            }
            let mut biases_a = o1[0].brain.biases[layer_to_crossover_neuron].col_mut(0);
            let mut biases_b = o2[0].brain.biases[layer_to_crossover_neuron].col_mut(0);

            for (i, w) in biases_a.iter_mut().zip(biases_b.iter_mut()).enumerate() {
                if i == neuron {
                    std::mem::swap(w.0, w.1);
                }
            }

            let mut weights_a = o1[0].brain.weights[layer_to_crossover_neuron].col_mut(neuron);
            let mut weights_b = o2[0].brain.weights[layer_to_crossover_neuron].col_mut(neuron);

            for w in weights_a.iter_mut().zip(weights_b.iter_mut()) {
                std::mem::swap(w.0, w.1);
            }

            already_done_neurons.push(neuron);
        }
    }

    offspring
}
