use crate::data::CSV;
use crate::data::Fasta;
use crate::data::Pathfile;
use serde_json::value;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

/*
 A machine learning approach which gives the expresion and the collection sample abundance and then
 associates the sequence information based on the unique kmer count as a classification
 to develop the features for the machine learning and classifes the sequences as potential
 association to the bees abundance and corresponding expression.

*/

impl CSV {
    /* the file should have all the data from the expression such as
     * the gene expression, bee swarming rates and the first column
     * should be the gene names.
     */

    fn load_dataset(&self) -> (DenseMatrix<f64>, Vec<i32>) {
        let file = File::open(self.pathcsv).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        let array: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();
        let (x, _) = array.split_at(Axis(1), array.ncols() - 1);
        let x_matrix = DenseMatrix::from_array(x.rows(), x.cols(), x.to_slice().unwrap());
        x_matrix
    }

    fn machinefit(&self) -> Result<String, Box<dyn Error>> {
        let xmatrix = self.load_dataset();
        let ymatrix = genomeread(pathfile, ids, kmer, threshold).unwrap();
        let predictmatrix = predict(pathfile, kmer, threshold).unwrap();
        let logistic = LogisticRegression::fit(xmatrix, ymatrix, Default::default()).unwrap();
        let predict = logistic.predict(x).unwrap();
        let accuracy = accuracy(predict, ymatrix);
        println!("The accuracy of the model is {}", accuracy);
    }
}

pub fn genomeread(
    pathfile: &str,
    ids: &str,
    kmer: &str,
    threshold: &str,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let filetypes: Vec<Fasta> = Vec::new();
    let mut seqid: Vec<String> = Vec::new();
    let mut seqvec: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            let id = line.split("\t").collect::<Vec<_>>()[0].to_string();
            seqid.push(id);
        }
        if !line.starts_with(">") {
            seqvec.push(line);
        }
        for i in 0..seqid.len() {
            filetypes.push(Fasta {
                name: seqid[i].clone().to_string(),
                seq: seqvec[i].clone().to_string(),
            })
        }
    }
    let finalseq = File::open(ids).expect("file not present");
    let finalread = BufReader::new(finalseq);
    let seqids: Vec<String> = Vec::new();
    for i in finalread.lines() {
        let line = i.expct("file not present");
        seqids.push(line);
    }

    let finalvector: Vec<Fasta> = Vec::new();
    for i in seqids.iter() {
        for val in filetypes.iter() {
            if i.to_string() == val.name {
                finalvector.push(Fasta {
                    name: val.name.clone(),
                    seq: val.seq.clone(),
                })
            }
        }
    }

    let mut abundancematrix: Vec<Vec<&str>> = Vec::new();
    for i in finalvector.iter() {
        let seqvec: Vec<&str> = i
            .seq
            .as_bytes()
            .windows(kmer.parse::<usize>().unwrap())
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        abundancematrix.push(seqvec);
    }
    let unwrapvec = abundancematrix
        .iter()
        .flatten()
        .cloned()
        .map(|x| s.to_string())
        .collect::<HashSet<String>>();

    let genomickmer: Vec<(String, usize)> = counthashes(unwrapvec, filetypes).unwrap();
    let valuetag: Vec<i32> = Vec::new();
    for i in genomickmer.iter() {
        if usize < threshold.parse::<usize>().unwrap() {
            valuetag.push(1usize);
        } else {
            valuetag.push(0usize);
        }
    }
    Ok(valuetag)
}

pub fn predict(pathfile: &str, kmer: &str, threshold: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let filetypes: Vec<Fasta> = Vec::new();
    let mut seqid: Vec<String> = Vec::new();
    let mut seqvec: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            let id = line.split("\t").collect::<Vec<_>>()[0].to_string();
            seqid.push(id);
        }
        if !line.starts_with(">") {
            seqvec.push(line);
        }
        for i in 0..seqid.len() {
            filetypes.push(Fasta {
                name: seqid[i].clone().to_string(),
                seq: seqvec[i].clone().to_string(),
            })
        }
    }

    let mut abundancematrix: Vec<Vec<&str>> = Vec::new();
    for i in finaltypes.iter() {
        let seqvec: Vec<&str> = i
            .seq
            .as_bytes()
            .windows(kmer.parse::<usize>().unwrap())
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        abundancematrix.push(seqvec);
    }
    let unwrapvec = abundancematrix
        .iter()
        .flatten()
        .cloned()
        .map(|x| s.to_string())
        .collect::<HashSet<String>>();

    let genomickmer: Vec<(String, usize)> = counthashes(unwrapvec, filetypes).unwrap();
    let valuetag: Vec<i32> = Vec::new();
    for i in genomickmer.iter() {
        if usize < threshold.parse::<usize>().unwrap() {
            valuetag.push(1usize);
        } else {
            valuetag.push(0usize);
        }
    }
    Ok(valuetag)
}

pub fn counthashes(
    pathhash: HashSet<String>,
    vecseq: Vec<Fasta>,
) -> Result<Vec<(String, usize)>, Box<dyn Error>> {
    let vector: Vec<Fasta> = vecseq;
    let vecreturn: Vec<(String, usize)> = Vec::new();
    for i in vector.iter() {
        count = 0usize;
        let seqvec: Vec<&str> = i
            .seq
            .as_bytes()
            .windows(3)
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        for i in seqvec.iter() {
            for val in pathhash.iter() {
                if i.clone().to_string() == val.clone() {
                    count += 1usize;
                }
            }
        }
        let countmake: (String, usize) = (i.name.clone(), count);
        vecreturn.push(countmake);
    }
}
