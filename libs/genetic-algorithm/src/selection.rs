pub use self::roulette_wheel::*;

use crate::*;

mod roulette_wheel;

// allows user to use any genetic selection algo they desire
pub trait SelectionMethod{
    fn select<'a, R, I>(&self, 
                    rng: &mut dyn RngCore, // dyn imply dynamic dispatch, without is static dispatch. IE we don't know what exactly what it will look like, like a lambda function in python
                    population: &'a [I],
                    ) -> &'a I // 'a annotates the output's lifetime
    where
        I: Individual;
}
