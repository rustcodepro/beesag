use crate::data::GenomeBee;
use crate::data::GenomeSeq;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

impl GenomeSeq {
    pub fn genomeparse(&self) -> Result<Vec<GenomeBee>, Box<dyn Error>> {
        let filepath = File::open(self.namefile).expect("file not present");
        let fileread = BufReader::new(filepath);
        let mut genomeaccession: Vec<GenomeBee> = Vec::new();
        for i in fileread.lines() {
            let line = i.expect("file not present");
            let linevec = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            genomeaccession.push(GenomeBee {
                skipline: linevec[0],
                id: linevec[1],
                accession: linevec[2],
                experiment: linevec[3],
                entries: linevec[4],
                host: linevec[5],
                organism: linevec[6],
                datecollect: linevec[7],
                location: linevec[8],
                source: linevec[9],
                layout: linevec[10],
                strategy: linevec[11],
                instrument: linevec[12],
                name: linevec[13],
                package: linevec[14],
            });
        }
        Ok(genomeaccession)
    }

    pub fn printaccession(&self) -> Result<String, Box<dyn Error>> {
        let filepathunravel = self.genomeparse().unwrap();
        for i in filepathunravel.iter() {
            println!("{}\t{}", i.accession, i.source);
        }
    }

    pub fn download(&self, accession: &str) -> Result<String, Box<dyn Error>> {
        let start: String =
            String::from("https://www.ebi.ac.uk/ena/portal/api/filereport?accession=");
        let middle: String = accession.to_string();
        let end: String = String::from(
            "&result=read_run&fields=study_accession,sample_accession,experiment_accession,run_accession,tax_id,scientific_name,fastq_ftp,submitted_ftp,sra_ftp,bam_ftp&format=tsv&download=true&limit=0",
        );
        let finals: String = format!("{}{}{}", start, middle, end);
        let project: String = accession.to_string();
        run_cmd!(wget $finals -O $project).expect("failed");
        let f = File::open(&project).expect("file not present");
        let read = BufReader::new(f);
        let mut filewrite = File::create("downloadbeegenome.sh").expect("file not present");
        for i in read.lines() {
            let line = i.expect("line not present");
            let linecapture: Vec<&str> = line.split('\t').collect();
            for i in &linecapture {
                if i.contains("ftp") && i.contains("gz") {
                    let intermediate: Vec<&str> = i.split(";").collect();
                    for i in &intermediate {
                        writeln!(filewrite, "wget {}", i).expect("file not present");
                    }
                }
            }
        }
        let _ = Command::new("sh")
            .arg("downloadbeegenome.sh")
            .output()
            .expect("command failed");
    }
}
