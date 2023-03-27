use colored::*;
use rust_htslib::bcf::{header::Header, Format, Read, Reader, Writer};
use rust_htslib::errors::Error;

pub fn select_sites(
    vcf: &Option<&str>,
    keep: &str,
    gzip: bool,
    out: &Option<&str>,
) -> Result<(), Error> {
    let mut bcf = if let Some(file) = vcf {
        Reader::from_path(file)?
    } else {
        Reader::from_stdin()?
    };

    let header1 = bcf.header();
    let header2 = Header::from_template(&header1);

    let mut bcf_writer = if let Some(file) = out {
        Writer::from_path(file, &header2, !gzip, Format::Vcf)?
    } else {
        Writer::from_stdout(&header2, !gzip, Format::Vcf)?
    };

    let mut flag_snp = 0;
    if keep == "SNP" {
        for rec in bcf.records() {
            if let Ok(ref info) = rec {
                for allele in info.alleles() {
                    if allele.len() == 1 {
                        continue;
                    } else {
                        flag_snp += 1;
                    }
                }
            }
            if flag_snp == 0 {
                bcf_writer.write(&rec?)?;
            }
            flag_snp = 0;
        }
    } else if keep == "INDEL" {
        for rec in bcf.records() {
            if let Ok(ref info) = rec {
                for allele in info.alleles() {
                    if allele.len() == 1 {
                        continue;
                    } else {
                        flag_snp += 1;
                    }
                }
            }
            if flag_snp != 0 {
                bcf_writer.write(&rec?)?;
            }
            flag_snp = 0;
        }
    } else {
        eprintln!(
            "{}",
            "[error]: invalid  arg '--keep', must be INDEL or SNP !".red()
        );
        std::process::exit(1);
    }

    Ok(())
}

