use num::Complex;
use crate::Qubit;

pub fn hadamard(qubit: &mut Qubit) -> Qubit {
    let root = Complex::<f64>::new(1f64/2f64.sqrt(), 0.0);

    *qubit.operate(root, root, root, -root)
}
