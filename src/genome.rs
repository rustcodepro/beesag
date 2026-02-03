use crate::data::Fasta;
use crate::data::Pathfile;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl Pathfile {
    pub fn genomeread(&self) -> Result<Vec<Fasta>, Box<dyn Error>> {
        let fileopen = File::open(self.pathname).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let mut fastaread: Vec<Fasta> = Vec::new();
        let mut id: Vec<String> = Vec::new();
        let mut value: Vec<String> = Vec::new();
        for i in fileread.lines() {
            let line = i.expect("line not present");
            if line.starts_with(">") {
                let linevec = line.split(">").collect::<Vec<_>>()[1];
                id.push(linevec);
            }
            if !line.starts_with(">") {
                value.push(line)
            }
        }
        for i in 0..id.len() {
            fastaread.push(Fasta {
                name: id[i].clone().to_string(),
                seq: value[i].clone().to_string(),
            })
        }
        Ok(fastaread)
    }

    pub fn extractseq(
        &self,
        name: &string,
        upstream: usize,
        downstream: usize,
    ) -> Result<String, Box<dyn Error>> {
        let filename = self.genomeread().unwrap();
        let resultvec: Vec<Fasta> = Vec::new();
        for i in filename.iter() {
            if i.name == name {
                let nameseq = i.name.clone();
                let seqseq: String = i.seq[upstream..downstream].to_string();
                resultvec.push(Fasta {
                    name: nameseq.clone(),
                    seq: seqseq.clone(),
                })
            }
        }
    }
}
