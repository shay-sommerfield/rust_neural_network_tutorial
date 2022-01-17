use rand::Rng;

// number of neurons per per layer
pub struct LayerTopology {
    pub neurons: usize,
}

pub struct Network {
    layers: Vec<Layer>,
}


impl Network {

    pub fn random(rng: &mut dyn rand::RngCore,layers: &[LayerTopology]) -> Self {

        // Network with just one layer is technically doable, but doesn't
        // make much sense:
        assert!(layers.len() > 1);
        
        //let mut built_layers = Vec::new();

        // for i in 0..(layers.len() - 1) {
        //     let input_neurons = layers[i].neurons;
        //     let output_neurons = layers[i + 1].neurons;
    
        //     built_layers.push(Layer::random(
        //         input_neurons,
        //         output_neurons,
        //     ));
        // }
    
        // Self { layers: built_layers }

        // switch to iterators. same as above ^^^
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(rng,layers[0].neurons, layers[1].neurons) // 0 must represent current layer and 1 next layer
            })
            .collect();

            Self{layers}

    }

    fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {

        // for layer in &self.layers {
        //     inputs = layer.propagate(inputs);
        // }
        // inputs

        self.layers
        .iter()
        .fold(inputs, |inputs, layer| layer.propagate(inputs))

    }
}

struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    pub fn random(rng: &mut dyn rand::RngCore,input_neurons: usize, output_neurons: usize) -> Self{
    
    //     let mut neurons = Vec::new(); // empty neuron vector

    //     for _ in 0..output_neurons {
    //         neurons.push(Neuron::random(input_neurons)); // create a new randomized neruon and push it to the list
    //     }

    // same as above ^^^
    let neurons = (0..output_neurons)  // create an iterator
        .map(|_| Neuron::random(rng,input_neurons)) // need to figure out what map does, but put a new neuron in it's vector place
        .collect(); // return the vector

        // the author noted theat the map function above could have been this: .map(|output_neuron_id| Neuron::random(input))
        // but since we don't use the output_neuron_id, we can just use |_|, called toilet closure
        // we could also have used _output_neuron_id, which would indicate that it isn't used. 
    Self { neurons } // assign neuron values to this layer? 
    }
    
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        //let mut outputs = Vec::with_capacity(self.neurons.len());

        // for neuron in &self.neurons {
        //     let output = neuron.propagate(&inputs);
        //     outputs.push(output);
        // }

        // outputs


        self.neurons
            .iter()  // creates an iterator that knows its length
            .map(|neuron| neuron.propagate(&inputs))
            .collect()  // creates a vector that's large enough
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn random(rng: &mut dyn rand::RngCore,
        output_size: usize) -> Self {
            // removed because we now input rng instead
        // let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
        .map(|_| rng.gen_range(-1.0..=1.0))
        .collect();

        Self { bias, weights }
    }
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let mut output = 0.0;
        
        // for i in 0..inputs.len() {
        //     output += inputs[i]*self.weights[i]
        // }

        // same as ^^^

        // increases speed becuase it prevents a bounds check from index access 
        //ie checking if length of x is >= 100 for x[100]
        // for (&input,&weight) in inputs.iter().zip(&self.weights){
        //     output += input*weight
        // }

        // same as ^^^

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>(); // explicitely states that the output should be <f32>

        //output += self.bias;

        // if output > 0.0 {
        //     output
        // } else {
        //     0.0
        // }
        
        // same as ^^^
        //output.max(0.0)

        (self.bias + output).max(0.0)
        
    }
}

// testing our randomizing algorithm
#[cfg(test)]
mod tests{
    use super::*; // access used/created  everything above the module

    mod random{
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test(){
            // Because we always use the same seed our rng in here
            // will always return the same set of values
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng,4);

            approx::assert_relative_eq!(neuron.bias,-0.6255188);
            approx::assert_relative_eq!(neuron.weights.as_slice(),&[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref());
    }

    }

    mod propagate{
        use super::*;

        #[test]
        fn test(){
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            // Ensures `.max()` (our ReLU) works:
            approx::assert_relative_eq!(
                neuron.propagate(&[-10.0, -10.0]),
                0.0,
            );
            
            // `0.5` and `1.0` chosen by a fair dice roll:
            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
            // We could've written `1.15` right away, but showing the entire
            // formula makes our intentions clearer
        }
    }

    // written by me (shay sommerfield)
    mod layer{
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn random_test(){
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 1, 1);

            approx::assert_relative_eq!(layer.neurons[0].bias,-0.6255188);
            approx::assert_relative_eq!(layer.neurons[0].weights.as_slice(),&[0.67383957].as_ref());
        }

        // #[test]
        // fn propagate_test(){
        //     let layer_top = LayerTopology()
        //     let layer = Layer
        // }
    }
    
    
}