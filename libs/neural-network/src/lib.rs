#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



pub struct Network {
    layers: Vec<Layer>,
}
impl Network {
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
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let mut output = 0.0;
        
        for i in 0..inputs.len() {
            output += inputs[i]*self.weights[i]
        }

        output += self.bias;

        if output > 0.0 {
            output
        } else {
            0.0
        }
        
    }
}