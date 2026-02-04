#[derive(Debug, Clone, PartialOrd, PartilEq)]
pub struct GenomeSeq {
    pub namefile: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenomeBee {
    pub skipline: String,
    pub id: String,
    pub accession: String,
    pub experiment: String,
    pub entries: String,
    pub host: String,
    pub organism: String,
    pub datecollect: String,
    pub location: String,
    pub source: String,
    pub layout: String,
    pub strategy: String,
    pub instrument: String,
    pub name: String,
    pub package: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Amel {
    pub mrna: Vec<(String, usize, usize)>,
    pub cds: Vec<(usize, usize)>,
    pub exon: Vec<(usize, usize)>,
    pub five_utr: Vec<(usize, usize)>,
    pub three_utr: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Bimp {
    pub mrna: Vec<(String, usize, usize)>,
    pub cds: Vec<(usize, usize)>,
    pub exon: Vec<(usize, usize)>,
    pub five_utr: Vec<(usize, usize)>,
    pub three_utr: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AmelSeq {
    pub mrna: String,
    pub mrnaseq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BimpSeq {
    pub mrna: String,
    pub mrnaseq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenomeSeq {
    pub pathfile: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fasta {
    pub name: String,
    pub seq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pathfile {
    pub pathname: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CSV {
    pub pathcsv: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenomeFasta {
    pub pathfile: String,
    pub expressionfile: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Protein {
    pub proteinfilename: String,
    pub expression: String,
}
