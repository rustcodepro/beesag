use crate::data::{Amel, Bimp};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

impl Amel {
    pub fn amelunique(&self, pathfile: &str) -> Result<HashMap<String>, Box<dyn Error>> {
        let pathfileopen = File::open(pathfile).expect("file not found");
        let fileopen = BufReader::new(pathfileopen);
        let amelvec: HashMap<String> = HashMap::new();
        for i in fileopen.lines() {
            let line = i.expect("file not present");
            let linevec = line
                .split("\t")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if linevec[2] == "mRNA" {
                let linesplit = linevec[8].split(";").collect::<Vec<_>>()[0].replace("ID=", "");
                amelvec.push(linesplit);
            }
        }
        Ok(amelvec)
    }

    pub fn amelparse(&self, pathfile: &str) -> Result<Vec<Amel>, Box<dyn Error>> {
        let hashmap = self.amelunique(pathfile).unwrap();
        let pathfileopen = File::open(pathfile).expect("file not found");
        let fileopen = BufReader::new(pathfileopen);
        let amelvec: HashMap<String> = HashMap::new();
        let mut mrna: Vec<(String, usize, usize)> = Vec::new();
        let mut cds:Vec<(String, Vec<(usize,usize)>)> = Vec::new();
        let mut exons: Vec<(String, Vec<(usize, usize)>)> = Vec::new();
        let mut five_utr:Vec<(String, Vec<(usize, usize)>)> = Vec::new();
        let mut three_utr:Vec<(String, Vec<(usize, usize)>)> = Vec::new();
        for i in fileopen.lines() {
            for val in hashmap.iter() {
                let line = i.expect("file not present");
                let linevec = line
                    .split("\t")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                if linevec[2] == "mRNA"
                    && linevec[8].split(";").collect::<Vec<_>>()[0].replace("ID=", "") == val
                {
                    mrna.push((
                        linevec[8].split(";").collect::<Vec<_>>()[0].replace("ID=", ""),
                        linevec[3].parse::<usize>().unwrap(),
                        linevec[4].parse::<usize>().unwrap(),
                    ))
                }
                if linevec[2] == "CDS" & linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", "") == val{
                 let mut valuevec:Vec<(usize, usize)> = Vec::new();
                 valuvec.push(linevec[3].parse::<usize>().unwrap(), linevec[4].parse::<usize>().unwrap());
                 cds.push((linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", ""), valuevec));
                }
                if linevec[2] == "exon" & linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", "") == val{
                 let mut valuevec:Vec<(usize, usize)> = Vec::new();
                 valuvec.push(linevec[3].parse::<usize>().unwrap(), linevec[4].parse::<usize>().unwrap());
                 cds.push((linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", ""), valuevec));
                }
                if linevec[2] == "five_prime_UTR" & linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", "") == val{
                 let mut valuevec:Vec<(usize, usize)> = Vec::new();
                 valuvec.push(linevec[3].parse::<usize>().unwrap(), linevec[4].parse::<usize>().unwrap());
                 cds.push((linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", ""), valuevec));
                }
                if linevec[2] == "three_prime_UTR" & linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", "") == val{
                 let mut valuevec:Vec<(usize, usize)> = Vec::new();
                 valuvec.push(linevec[3].parse::<usize>().unwrap(), linevec[4].parse::<usize>().unwrap());
                 cds.push((linevec[8].split(";").collect::<Vec<_>>()[0].replace("Parent=", ""), valuevec));
                }
            }
        }
         let mut finalvec:Vec<Bimp> = Vec::new();
         for i in mrna.iter(){
         for val in cds.iter(){
          for valex in exons.iter(){
          for utrseq in five_utr.iter(){
            for threeseq in three_utr.iter(){
               if i.0 == val.0 && i.0 == valex.0 && i.0  == utrseq.0 && i.0 == threeseq.0 {
               finalvec.push(Amel{mrna: i, cds: val, exon: valex, five_utr: utrseq, three_utr: threeseq});
               }
            }
          }
          }
         }
         }
    }
}
