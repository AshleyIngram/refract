use std::cell::RefCell;

use rand::{
    RngExt,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
    rngs::SmallRng,
};

thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(rand::make_rng());
}

pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    RNG.with(|rng| rng.borrow_mut().random_range(range))
}

pub fn random<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    RNG.with(|rng| rng.borrow_mut().random())
}

#[cfg(test)]
pub fn reseed(seed: u64) {
    use rand::SeedableRng;
    RNG.with(|rng| *rng.borrow_mut() = SmallRng::seed_from_u64(seed));
}
