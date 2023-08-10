use anyhow::Result;
use clap::Parser;

mod select;
mod split;
mod stats;
mod concat;
use select::*;
use split::*;
use stats::*;
use concat::*;

#[derive(Parser, Debug)]
#[command(
    author = "size_t",
    version = "version 0.1.1",
    about = "vcfkit: a simple program for vcf/bcf file manipulation",
    long_about = "vcfkit: a simple program for vcf/bcf file manipulation\nrepo: https::github.com/sharkLoc/vcfkit.git\nemail: dwood8146@gmail.com"
)]
struct Args {
    #[clap(subcommand)]
    command: Subcli,
}

#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
enum Subcli {
    /// concatenate VCF/BCF files from the same set of samples
    concat {
        /// input VCF/BCF file list, containing the path to the files, one per line and no blank line.
        #[arg(short= 'l', long = "list")]
        list: String,

        /// if specified, output bgzip compressed file.
        #[arg(short = 'z', long = "gzip")]
        gzip: bool,

        /// output vcf file name, or write to stdout.
        out: Option<String>,
    },

    /// convert BCF file to VCF file formats
    convert {
        /// input bcf file, or read from stdin.    
        bcf: Option<String>,
        
        /// if specified, output bgzip compressed file.
        #[arg(short = 'z', long = "gzip")]
        gzip: bool,

        /// output vcf file name, or write to stdout.
        #[arg(short = 'o', long = "out")]
        out : String
    },
    
    /// select SNP/INDEL variants from a VCF file
    select {
        /// input vcf[.gz] file, or read from stdin.
        vcf: Option<String>,

        /// select variant type: SNP/INDEL
        #[arg(short = 'k', long = "keep", default_value_t = String::from("SNP") )]
        keep: String,

        /// if specified, output bgzip compressed file.
        #[arg(short = 'z', long = "gzip")]
        gzip: bool,

        /// output vcf file name, or write to stdout.
        out: Option<String>,
    },
    
    /// split vcf file by chromosome name
    split {
        /// input vcf[.gz] file, or read from stdin.
        vcf: Option<String>,

        /// split vcf file output dir.
        #[arg(short = 'o', long = "out")]
        outdir: String,
    },
    
    ///statistics including alleles frequence, missing rate, etc.
    stats {
        /// input vcf[.gz] file, or read from stdin.
        vcf: Option<String>,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let arg = Args::parse();
    match arg.command {
        Subcli::select {
            vcf,
            keep,
            gzip,
            out,
        } => match vcf {
            Some(v) => {
                if let Some(o) = out {
                    let _x = select_sites(&Some(&v), &keep, gzip, &Some(&o))?;
                } else {
                    let _x = select_sites(&Some(&v), &keep, gzip, &None)?;
                }
            }
            None => {
                if let Some(o) = out {
                    let _x = select_sites(&None, &keep, gzip, &Some(&o))?;
                } else {
                    let _x = select_sites(&None, &keep, gzip, &None)?;
                }
            }
        }
        Subcli::split {
            vcf,
            outdir,
        } => match vcf {
            Some(v) => { let _x = split_by_chr(&Some(&v), &outdir)?; }
            None => { let _x = split_by_chr(&None, &outdir)?; }
        }
        Subcli::concat {
            list,
            gzip,
            out
        } => match out {
            Some(x) => { let _x = concat_vcfs(&list, gzip, &Some(&x))?;}
            None => { let _x = concat_vcfs(&list, gzip, &None)?; }
        }
        Subcli::convert {
            bcf,
            gzip,
            out
        } => match bcf {
            Some(v) => { let _x = bcf2vcf(&Some(&v), gzip, &out)?; }
            None => { let _x = bcf2vcf(&None, gzip, &out)?; }
        }
        Subcli::stats {vcf} => match vcf {
            Some(v) => { let _x =  calc_sites(&Some(&v));}
            None => { let _x = calc_sites(&None);}
        }
    }
    Ok(())
}
