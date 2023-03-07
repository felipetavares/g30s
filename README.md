# Quantum in 30 seconds

Classical simulation of basic quantum computation can be pretty simple, even
when including the underlying minimal linear algebra. The trick is to introduce
the subject without entanglement at first.

# Experiments

Here, we run a couple experiments to show what happens when you measure the
state of a qubit several times in a row, and what happens when you measure the
state of several different qubits in a row.

The qubit in question is initialized to a *superposition state* where it has a
50% probability of being measured as 0 and 50% as 1. Superposition just means
the state space of qubits is bigger than the measurable state space, or in
other words, there's more information in the qubit than it is measurable in a
single measurement. If you could do more measurements you would get more
information about the state of the qubit each time, but turns out qubits lose
their state after you measure them, so to extract the state of a qubit you
would need to initialize several of them to the same state and measure each one
independently.

So our two experiments should look like this:

1. When measuring a single qubit, it is going to collapse after the first
   measurement to either 0 or 1. Subsequent measurements are always going to
   result in the same outcome.
2. When measuring several differnet qubits 

# Outcomes

Indeed, the results are as expected:

```python
❯ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/q30s`
# 1. Always results in the same outcome 
[src/main.rs:22] generate_distribution(results) = [
    100,
    0,
]
# 2. ~50% chance for each outcome 
[src/main.rs:29] generate_distribution(results) = [
    40,
    60,
]
```
