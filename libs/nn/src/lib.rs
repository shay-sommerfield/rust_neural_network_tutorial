use rand::Rng;

struct Neuron{
    bias: f32,
    weights: Vec<f32>,
}
impl Neuron{
    // multiply each input by it's corresponding weight and add the bias. 
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(),self.weights.len()); //need the same amount of inputs as weights

        // multiply each input by it's corresponding weight and sum all into an output
        
        //----V1-------
        //let mut output = 0.0;
        // for i in 0..inputs.len(){
        //     output += inputs[i]*self.weights[i];
        // }
        
        //---V2-----
        // make ait look more rustic, also avoid using iterator to avoid a bounds check
        // for (&input,&weight) in inputs.iter().zip(%&self.weights){
        //     output += input*weight;
        // }

        // output += self.bias;

        // if output > 0.0{
        //     output
        // }else{
        //     0.0
        // }

        //------V3-------
        let output = inputs
            .iter() // create an iterator
            .zip(&self.weights) // zip inputs and weights togerther since they are the same length
            .map(|(input,weight)| input*weight) // map input*weight to the iterator for each output of (input,weight) by the iterator
            .sum::<f32>(); // sum the output into an f32

            (self.bias + output).max(0.0) // return the output plus the bias or 0.0, whichever is greater
    }
    fn random(input_size: usize) -> Self{
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

            Self{bias,weights}
    }
}
struct LayerTopology{
    pub input_neurons: usize,
    pub output_neurons: usize,
}

struct Layer{
    neurons: Vec<Neuron>,
}
impl Layer{
    // propagate each neuron's inputs into a vector of outputs
    // it should be noted that each neurons inputs will look the same since
    // each neuron can only have one output,it will feed the same output N time for N neurons in the next layer
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32>{
        let mut outputs = Vec::new();

        for neuron in &self.neurons{
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }

        outputs
    }
    fn random(input_neurons: usize, output_neurons:usize) -> Self{
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons))
            .collect();

        Self{neurons}
    }
}

struct Network{
    layers: Vec<Layer>
}

impl Network{
    // propagate each layer through and return the last output out of the network
    fn propagate(&self,mut inputs: Vec<f32>) -> Vec<f32>{
        // for layer in self.layers{
        //     inputs = layer.propagate(inputs);
        // }
        self.layers
            .iter()
            .fold(inputs, |inputs,layer| layer.propagate(inputs))
            // for fold(x, |y,z| y+z) is the same as inputs on the left is the starting value, if it was 0, inputs would start at 0
            // inputs doesn't need to be returned because fold will return the inputs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
