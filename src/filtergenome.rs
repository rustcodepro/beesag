use crate::data::GenomeFasta;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FastaReader {
    pub head: String,
    pub seq: String,
}

type VECRETURN = (Vec<FastaReader>, Vec<i32>);

pub fn selectedseq(&self, threshold: &str) -> Result<VECRETURN, Box<dyn Error>> {
    let fileopen = File::open(self.pathfile).expect("expect file not present");
    let fileread = BufReader::new(fileopen);
    let expressionfile = File::open(self.expressionfile).expect("file not present");
    let expressionread = BufReader::new(expressionfile);
    let id: Vec<String> = Vec::new();
    let seq: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            id.push(line.split(">").collect::<Vec<_>>()[1].to_string());
        }
        if !line.starts_with(">") {
            seq.push(line);
        }
    }
    let mut finalvec: Vec<FastaReader> = Vec::new();
    for i in 0..id.len() {
        finalvec.push(FastaReader {
            head: id[i].clone(),
            seq: seq[i].clone(),
        })
    }

    let mut selectedones: Vec<FastaReader> = Vec::new();
    let mut genomeids: Vec<String> = Vec::new();
    let mut expressionvalue: Vec<usize> = Vec::new();
    for i in expressionread.lines() {
        let line = i.expect("line not present");
        let linevec = line.split("\t").collect::<Vec<_>>();
        genomeids.push(linevec[0].to_string());
        expressionvalue.push(linevec[1].parse::<usize>().unwrap());
    }

    let selectedfasta: Vec<FastaReader> = Vec::new();
    for i in genomeids.iter() {
        for val in finalvec.iter() {
            if val.head == i {
                selectedfasta.push(FastaReader {
                    head: val.head,
                    seq: val.seq,
                });
            }
        }
    }

    let mut ymatrix: Vec<i32> = Vec::new();

    for i in expressionvalue.iter() {
        if i < threshold.parse::<usize>().unwrap() {
            ymatrix.push(0usize);
        } else if i > threshold.parse::<usize>().unwrap() {
            ymatrix.push(1usize);
        } else {
            continue;
        }
    }

    let returnvec: (Vec<FastaReader>, Vec<i32>) = (selectedfasta, ymatrix);
    returnvec
}
