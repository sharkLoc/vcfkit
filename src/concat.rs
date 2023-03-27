use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;
use rust_htslib::bcf::{header::Header, Format, Read, Reader, Writer};

pub fn concat_vcfs(list: &str, gzip: bool, out: &Option<&str>) -> Result<(), anyhow::Error> {
    let reader = BufReader::new(File::open(list)?);
    let mut list = vec![];
    for line in reader.lines().flatten() {
        list.push(line.to_string());
    }
    let bcf_reader = Reader::from_path(Path::new(list.get(0).unwrap()))?;
    let header = Header::from_template(&bcf_reader.header());

    let mut bcf_writer = if let Some(file) = out {
        Writer::from_path(file, &header, !gzip, Format::Vcf)?
    } else {
        Writer::from_stdout(&header, !gzip, Format::Vcf)?
    };
    let mut n = 0;
    for vcf in list {
        n += 1;
        let mut bcf = Reader::from_path(Path::new(&vcf))?;
        eprintln!("[info]: processing {n}th file {}", &vcf);
        for rec in bcf.records() {
            bcf_writer.write(&rec?)?;
        }
    }

    Ok(())
}

pub fn bcf2vcf(
    bcf: &Option<&str>,
    gzip: bool,
    out: &str
) -> Result<(), anyhow::Error> {
    let mut bcf_reader = if let Some(file) = bcf {
        Reader::from_path(file)? 
    } else {
        Reader::from_stdin()?   
    };
    let header = Header::from_template(&bcf_reader.header());

    let mut bcf_writer = Writer::from_path(Path::new(&out), &header, !gzip, Format::Vcf)?;
    for rec in bcf_reader.records() {
        bcf_writer.write(&rec?)?;    
    }

    Ok(())
}
