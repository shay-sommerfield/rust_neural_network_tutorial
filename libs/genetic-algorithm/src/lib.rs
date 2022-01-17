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
