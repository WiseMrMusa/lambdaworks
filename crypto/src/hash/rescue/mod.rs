use self::parameters::Parameters;
use lambdaworks_math::field::{traits::IsField, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField};

mod parameters;

pub struct Rescue<F: IsField> {
    params: Parameters<F>,
}

impl Rescue<Stark252PrimeField> {
    pub fn new() -> Self {
        Self { params: Parameters::<Stark252PrimeField>::new().expect("Error loading parameters") }
    }
}
