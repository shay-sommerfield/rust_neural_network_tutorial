

pub struct Network {
    layers: Vec<Layer>,
}
impl Network {
    // a non-mutable input can be passed to a function that requires a mutable argument
    pub fn propagate(&self,mut inputs: Vec<f32>) -> Vec<f32>{

        // for layer in &self.layers {
        //     inputs = layer.propagate(inputs);
        // }

        // The following is the same as the above for loop
        self.layers.iter().fold(inputs, |inputs, layer| layer.propagate(inputs))

        inputs
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn propagate(&self,inputs: Vec<f32>) -> Vec<f32>{
        todo!()
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
impl Neruon { 
    fn propagate(&self, inputs:Vec<f32>) -> f32 {
        todo!()
    }
}