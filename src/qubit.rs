use num::Complex;
use rand::Rng;

type C64 = Complex<f64>;

#[derive(Copy, Clone, Debug)]
pub struct Qubit(C64, C64);

pub const KET0: Qubit = Qubit(C64::new(1.0, 0.0), C64::new(0.0, 0.0));
pub const KET1: Qubit = Qubit(C64::new(0.0, 0.0), C64::new(1.0, 0.0));

impl Qubit {
    fn project(&self, other: Qubit) -> Complex<f64> {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn operate(&mut self, a: C64, b: C64, c: C64, d: C64) -> &Qubit {
        let x = self.0 * a + self.1 * b;
        let y = self.0 * c + self.1 * d;

        self.0 = x;
        self.1 = y;

        self
    }
 
    pub fn measure(&mut self) -> bool {
        let projection = self.project(KET1);
        let probability = projection.norm() * projection.norm();

        if rand::thread_rng().gen_bool(probability) {
            *self = KET1;
        } else {
            *self = KET0;
        }

        *self == KET1
    }
}

impl PartialEq for Qubit {
    fn eq(&self, other: &Qubit) -> bool {
        const EPSILON: f64 = 0.001;

        (self.0.im-other.0.im).abs() < EPSILON &&
        (self.0.re-other.0.re).abs() < EPSILON &&
        (self.1.im-other.1.im).abs() < EPSILON &&
        (self.1.re-other.1.re).abs() < EPSILON
    }
}

impl Eq for Qubit {}
