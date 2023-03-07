use std::iter::repeat_with;
use qubit::Qubit;

mod qubit;
mod operators;

fn generate_distribution(experiment: Vec<bool>) -> Vec<usize> {
    experiment.into_iter().fold(vec![0, 0], |mut acc, qubit| {
        if qubit { acc[0] += 1 } else { acc[1] += 1 }
        acc
    })
}

fn main() {
    const EXPERIMENT_COUNT: usize = 100;

    // 1. Same Qubit - 100% either 0/1 due to wave function collapse 
    let mut qubit = operators::hadamard(&mut qubit::KET0.clone());
    let results: Vec<bool> = repeat_with(|| qubit.measure())
        .take(EXPERIMENT_COUNT)
        .collect();
    dbg!(generate_distribution(results));

    // 2. Independent Qubits - 50% 0, 50% 1
    let qubit = operators::hadamard(&mut qubit::KET0.clone());
    let results: Vec<bool> = repeat_with(|| qubit.clone().measure())
        .take(EXPERIMENT_COUNT)
        .collect();
    dbg!(generate_distribution(results));
}
