use crate::data::Fasta;
use crate::data::GenomeSeq;
use smartcore::linear::linear_regression::LinearRegression;
use smartcore::linear::logistic_regression::LogisticRegression;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl GenomeSeq {
    pub fn genomemine(
        &self,
        region: usize,
        regionend: usize,
    ) -> Result<Vec<Fasta>, Box<dyn Error>> {
        let fileopen = File::open(self.pathfile).expect("file not present");
    }
}
