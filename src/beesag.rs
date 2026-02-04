use crate::data::Protein;
use candle::{Optimizer, Tensor};
use ndarray::Array1;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl Protein {
    fn proteinseq(&self, learning: &str, epoch: &str) -> Result<String, Box<dyn Error>> {
        let learningrate = learning.parse::<f32>().unwrap();
        let epochs = epoch.parse::<usize>().unwrap();
        fn create_network() -> MyNetwork {
            MyNetwork {
                conv_layer: Tensor::from(vec![0.2, 0.5, 0.3]).view([1, 3]),
            }
        }
        struct MyNetwork {
            conv_layer: Tensor,
        }
        impl MyNetwork {
            fn forward(&self, input: Tensor) -> Tensor {
                let output = input.conv1d(self.conv_layer.clone(), 1, 0).unwrap();
                output
            }
            fn backward(&self, output: &Tensor, target: f32) {
                // Placeholder for backpropagation logic
                // Use the gradients to update weights if needed
            }
        }
        fn prepare_input(sequence: &str) -> Tensor {
            let amino_acid_map = create_amino_acid_map();
            let input: Vec<f32> = sequence
                .chars()
                .map(|aa| *amino_acid_map.get(&aa).unwrap_or(&0.0))
                .collect();

            Tensor::from(input).view([-1, 1]) // 1D tensor
        }
        // Create mapping of amino acids to numerical values
        fn create_amino_acid_map() -> HashMap<char, f32> {
            let mut map = HashMap::new();
            let amino_acids = preparedataset(&self)
                .unwrap()
                .iter()
                .map(|x| x.0)
                .collect::<Vec<String>>();
            for i in amino_acids.iter() {
                for (index, aa) in i.chars().enumerate() {
                    map.insert(aa, index as f32 + 1.0); // Assign a numerical value starting from 1
                }
            }
            map
        }
        fn compute_loss(output: &Tensor, target: f32) -> Tensor {
            let output_value = output.item();
            let loss = (output_value - target).powi(2); // Mean Squared Error
            Tensor::from(loss)
        }
        let data = preparedataset(self).unwrap();
        let mut optimizer = candle::Adam::default().learning_rate(LEARNING_RATE);
        let net = create_network();
        for epoch in 0..EPOCHS {
            let mut total_loss = 0.0;
            for (sequence, target) in data {
                let input_tensor = prepare_input(sequence);
                // Forward pass
                let output = net.forward(input_tensor.clone());
                // Compute loss
                let loss = compute_loss(&output, *target);
                total_loss += loss.item();
                // Backward pass and optimization
                optimizer.zero_grad();
                net.backward(&output, *target);
                optimizer.step();
            }
            println!(
                "Epoch {}: Loss: {:.4}",
                epoch + 1,
                total_loss / DATA.len() as f32
            );
        }
    }
}

pub fn preparedataset(&self) -> Result<Vec<Vec<(String, f32)>>, Box<dyn Error>> {
    let fileopen = File::open(self.proteinfilename).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let proteinvec: Vec<(String, usize)> = Vec::new();
    let mut proteinid: Vec<String> = Vec::new();
    let mut proteinseq: Vec<Seq> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            proteinid.push(line.split(">").collect::<Vec<_>>()[1].to_string())
        }
        if !line.starts_with(">") {
            proteinseq.push(line)
        }
    }
    let expressionread = File::open(self.expression).expect("file not present");
    let expressionreadfinal = BufReader::new(expressionread);
    let mut expressionsec: Vec<f32> = Vec::new();
    for i in expressionreadfinal.lines() {
        let line = i.expect("file not present");
        expressionsec.push(line.parse::<f32>().unwrap());
    }
    let finalvector: Vec<(String, f32)> = Vec::new();
    for i in expressionsec.iter() {
        for val in proteinseq.iter() {
            finalvector.push(&vec![(val.clone, i)]);
        }
    }
    Ok(finalvector)
}
