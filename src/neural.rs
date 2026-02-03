use crate::data::GenomeFasta;
use crate::filtergenome::selectedseq;
use burn::Tensor as BurnTensor;
use burn::module::Module;
use burn::nn::activation::Relu;
use burn::optim::{AdamConfig, Optimizer};
use burn::prelude::*;
use burn::tensor::TensorData;
use burn::tensor::backend::{AutodiffBackend, Backend};
use std::error::Error as NormError;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

/*
 bee specific encoder and expression specific encoder for trainig on the expression based sequences.
*/

#[derive(burn::config::Config, Debug)]
pub struct AutoencoderConfig {
    pub inputsize: usize,
    pub hiddensize: usize,
    pub latentsize: usize,
    pub outputsize: usize,
}

#[derive(Debug, Module)]
pub struct Autoencoder<B: Backend> {
    encoder_1: nn::Linear<B>,
    encoder_2: nn::Linear<B>,
    decoder_1: nn::Linear<B>,
    decoder_2: nn::Linear<B>,
}

#[derive(Debug, Clone)]
pub struct PTMData<B: Backend> {
    pub inputdata: Tensor<B, 2>,
    pub outputdata: Tensor<B, 2>,
}

impl AutoencoderConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Autoencoder<B> {
        Autoencoder {
            encoder_1: nn::LinearConfig::new(self.inputsize, self.hiddensize).init(device),
            encoder_2: nn::LinearConfig::new(self.hiddensize, self.latentsize).init(device),
            decoder_1: nn::LinearConfig::new(self.latentsize, self.hiddensize).init(device),
            decoder_2: nn::LinearConfig::new(self.hiddensize, self.inputsize).init(device),
        }
    }
}

impl<B: Backend> Autoencoder<B> {
    fn forward(&self, input: BurnTensor<B, 2>) -> BurnTensor<B, 2> {
        let encoded = self.encoder_1.forward(input.clone());
        let activation = Relu::new().forward(encoded);
        let layeradd = self.encoder_2.forward(activation);
        let decoderlayer = self.decoder_1.forward(layeradd);
        let activate = Relu::new().forward(decoderlayer);
        let finalvalue = self.decoder_2.forward(activate);
        finalvalue
    }
}

pub fn genome_encode<B: Backend>(
    pathfile: &str,
    device: &B::Device,
) -> Result<BurnTensor<B, 2>, Box<dyn NormError>> {
    let sequenceveciter = selectedseq(&self, threshold).unwrap();
    let sequencevec = sequenceveciter.0;
    let mut sequencefinalvec: Vec<Vec<Vec<f32>>> = Vec::new();
    for i in sequencevec.iter() {
        let valueinsert = i.chars().collect::<Vec<char>>();
        let mut valuevec: Vec<Vec<f32>> = Vec::new();
        for i in valueinsert.iter() {
            match i {
                'A' => valuevec.push(vec![1.0, 0.0, 0.0, 0.0]),
                'T' => valuevec.push(vec![0.0, 1.0, 0.0, 0.0]),
                'G' => valuevec.push(vec![0.0, 0.0, 1.0, 0.0]),
                'C' => valuevec.push(vec![0.0, 0.0, 0.0, 1.0]),
                'N' => valuevec.push(vec![1.1, 1.1, 1.1, 1.1]),
                _ => continue,
            }
        }
        sequencefinalvec.push(valuevec);
    }
    let mut finaltensor = vec![];
    for i in sequencefinalvec.iter() {
        let rows = i.len();
        let col = i[0].len();
        let tensorflat = i.iter().flatten().cloned().collect::<Vec<f32>>();
        let tensorsearch = TensorData::new(tensorflat, Shape::new([rows as usize, col as usize]));
        let tensorreshape: Tensor<B, 2> = BurnTensor::from_data(tensorsearch, device);
        finaltensor.push(tensorreshape);
    }
    let finaltensor = BurnTensor::stack(finaltensor, 0);
    Ok(finaltensor)
}

pub fn train_autoencoder<B: AutodiffBackend>(
    seqlen: &str,
    hiddensizevalue: &str,
    latentsizeinput: &str,
    pathfile: &str,
    epochscount: &str,
) -> Result<String, Box<dyn NormError>> {
    let config = AutoencoderConfig::new(
        seqlen.parse::<usize>().unwrap() * 4,
        hiddensizevalue.parse::<usize>().unwrap(),
        latentsizeinput.parse::<usize>().unwrap(),
        seqlen.parse::<usize>().unwrap() * 4,
    );
    let mut model: Autoencoder<B> = config.init(Default::default());
    let mut optimizer = AdamConfig::new().init();
    let datainput = genome_encode(pathfile, Device::default()).unwrap();
    for epoch in 0..epochscount.parse::<usize>().unwrap() {
        let output = model.forward(datainput.clone());
        let lossvalue = burn::nn::loss::MseLoss::new().forward(
            output,
            datainput.clone(),
            burn::nn::loss::Reduction::Mean,
        );
        let grads = lossvalue.backward();
        let gradsvalue = burn::optim::GradientsParams::from_grads(grads, &model);
        model = optimizer.step(0.001, model, gradsvalue);
        println!("Epoch {}: Loss: {:.4}", epoch, lossvalue.into_scalar());
    }
    Ok("The bee autoencoder has been written".to_string())
}
