use crate::*;

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection{
    fn select<'a, I>(
        &self,
        rng: &mut dyn RngCore, // dyn imply dynamic dispatch, without is static dispatch. IE we don't know what exactly what it will look like, like a lambda function in python
        population &'a [I],
    ) -> &'a I                 // 'a annotates the output's lifetime
    where
        I:Individual,
        {
            poplation.choose_weighted(rng, |individual|,individual.fitness())
            .expect("got empty population")
        } 
}

#[cfg(test)]
mod tests{
    use super::*;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![ /* what here? */ ];

        let actual = RouletteWheelSelection::new()
            .select(&mut rng, &population);

        assert!(/* what here? */);
    }
    }