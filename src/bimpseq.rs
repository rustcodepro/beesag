use crate::data::BimpSeq;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fasta {
    pub name: String,
    pub seq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ReturnVec {
    pub name: String,
    pub seq: String,
    pub start: usize,
    pub end: usize,
}

impl BimpSeq {
    pub fn bimpseq(&self, pathfile: &str, seqfile: &str) -> Result<Vec<ReturnVec>, Box<dyn Error>> {
        let fileopen = File::open(pathfile).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let seqfileopen = File::open(seqfile).expect("file not present");
        let seqfileread = BufReader::new(seqfileopen);
        let mut hashids: Vec<(String, usize, usize)> = Vec::new();
        let mut idvec: Vec<String> = Vec::new();
        let mut seqvec: Vec<String> = Vec::new();
        let mut finalvec: Vec<AmelSeq> = Vec::new();
        for i in fileread.lines() {
            let line = i.expect("file not present");
            let linevec = line.split("\t").collect::<Vec<_>>();
            if linevec[2] == "mRNA" {
                let value = linevec[8].split(";").collect::<Vec<_>>()[1].replace("Parent=", "");
                hashids.push((
                    value,
                    linevec[3].parse::<usize>().unwrap(),
                    linevec[4].parse::<usize>().unwrap(),
                ));
            }
        }
        for i in seqfileread.lines() {
            let line = i.expect("file not present");
            if line.starts_with(">") {
                idvec.push(line.split("|").collect::<Vec<_>>()[2].to_string());
            }
            if !line.starts_with(">") {
                seqvec.push(line.to_string());
            }
        }

        let mut combinevec: Vec<Fasta> = Vec::new();
        for i in idvec.iter() {
            for val in seqvec.iter() {
                combinevec.push(Fasta {
                    name: i.clone(),
                    seq: val.clone(),
                })
            }
        }

        let mut finalvec: Vec<ReturnVec> = Vec::new();
        for i in combinevec.iter() {
            for val in hashids.iter() {
                if val.0 == i.name {
                    finalvec.push(ReturnVec {
                        name: i.name.clone(),
                        seq: i.seq.clone(),
                        start: val.1,
                        end: val.2,
                    })
                }
            }
        }

        Ok(finalvec)
    }
}
