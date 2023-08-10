use std::collections::HashMap;
use std::path::Path;

use rust_htslib::{
    bcf::{header::Header, Format, Read, Reader, Writer},
    errors::Error,
};

pub fn split_by_chr(
    vcf: &Option<&str>, 
    outdir: &str
) -> Result<(), Error> {
    let mut bcf = if let Some(file) = vcf {
        Reader::from_path(file)?
    } else {
        Reader::from_stdin()?
    };
    let header = Header::from_template(&bcf.header());
    let mut chr_hash = HashMap::new();

    for rec in bcf.records() {
        let chr: String = if let Ok(ref record) = rec {
            let name = String::from_utf8(record.header().rid2name(record.rid().unwrap())?.to_vec())
                .unwrap();
            if !chr_hash.contains_key(&name){
                let writer = Writer::from_path(
                    Path::new(&format!("{}/{}.vcf.gz", outdir, name)),
                    &header, 
                    false,
                    Format::Vcf,
                );
                chr_hash.insert(name.clone(), writer);
            }
            name
        } else {
            eprintln!(
                "[error]: fail to get chromosome name at pos: {}",
                rec.unwrap().pos() + 1
            );
            std::process::exit(1);
        };
        if let Some(bcf_writer) = chr_hash.get_mut(&chr) {
            //if let Ok(w) = bcf_writer {  w.write(&rec?)?; }
            bcf_writer.as_mut().unwrap().write(&rec?)?;
        }
    }
    Ok(())
}
