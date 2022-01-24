// overall lib
pub struct GeneticAlgorithm;

impl GeneticAlgorithm{
    // contructor
    pub fn new() -> Self {
        Self
    }

    // evolve the whole population
    pub fn evolve<I>(&self,
                     population: &[I]) -> Vec<I> { // I stands for individual, and we are creatinga  gn
        assert!(!population.is_empty()); // can't input an empty population

        (0..population.len())
            .map(|_|{
                //selection
                // corssover
                //mutation
                todo!()
            })
            .collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

use rand::RngCore;

pub trait SelectionMethod {
    fn select<'a, I>(
       &self,
       rng: &mut dyn RngCore,
       population: &'a [I],
    ) -> &'a I
    where
        I: Individual;
}



use rand::Rng;
use rand::seq::SliceRandom;

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}


impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(
       &self,
       rng: &mut dyn RngCore,
       population: &'a [I],
    ) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }    
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::iter::FromIterator;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());
    
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];
    
        let mut actual_histogram = BTreeMap::new();
    
        //               there is nothing special about this thousand;
        //          v--v a number as low as fifty might do the trick, too
        for _ in 0..1000 {
            let fitness = method
                .select(&mut rng, &population)
                .fitness() as i32;
    
            *actual_histogram
                .entry(fitness)
                .or_insert(0) += 1;
        }
    
        let expected_histogram = BTreeMap::from_iter(vec![
            // (fitness, how many times this fitness has been chosen)
            (1, 98),
            (2, 202),
            (3, 278),
            (4, 422),
        ]);
    
        assert_eq!(actual_histogram, expected_histogram);
    }
}