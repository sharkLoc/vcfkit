use rust_htslib::bcf::{Read, Reader};
use rust_htslib::errors::Error;

#[derive(Debug)]
struct ALLELES {
    count: Vec<Vec<usize>>,
}

impl ALLELES {
    fn new(sample_count: usize, allele_count: usize) -> Self {
        ALLELES {
            count: vec![vec![0; sample_count]; allele_count],
        }
    }

    fn row_sum(&self, allele_count_idx: usize) -> usize {
        self.count[allele_count_idx].iter().sum::<usize>()
    }
    // calc site miss rate
    fn miss_rate(&self, allele_count: usize) -> f64 {
        let mut total = 0;
        for r in 0..=allele_count {
            total += self.row_sum(r);
        }
        self.row_sum(allele_count) as f64 / total as f64
    }
    // calc ref allele frequence
    fn ref_af(&self, allele_count: usize) -> f64 {
        let mut total = 0;
        for r in 0..allele_count {
            total += self.row_sum(r);
        }
        self.row_sum(0) as f64 / total as f64
    }
    // calc alt alleles frequence
    fn alts_af(&self, allele_count: usize) -> Vec<f64> {
        let mut alts: Vec<f64> = vec![];
        let mut total = 0;
        for r in 0..allele_count {
            total += self.row_sum(r);
        }
        for alt in 1..allele_count {
            alts.push(self.row_sum(alt) as f64 / total as f64);
        }
        alts
    }
}

pub fn calc_sites(
    vcf: &Option<&str>,
    //out: &Option<&str>,
) -> Result<(), Error> {
    let mut bcf = if let Some(file) = vcf {
        Reader::from_path(file)?
    } else {
        Reader::from_stdin()?
    };
    let header = bcf.header();
    let sample_count = header.sample_count() as usize;

    println!("chrom\tpos\tqual\tallele_count\tmiss_rate\tref:af\talt:af");
    for rec in bcf.records() {
        let mut alleles: Vec<String> = vec![]; // vec![ref, alt1, alt2 ...]
        if let Ok(ref row) = rec {
            for allele in row.alleles() {
                let mut each = String::new();
                for nt in allele {
                    each.push(char::from(*nt))
                }
                alleles.push(each);
            }
            let pos = row.pos() + 1;
            let allele_count = row.allele_count() as usize;

            let mut geno_count: ALLELES = ALLELES::new(sample_count, allele_count + 1);
            let genotypes = row.genotypes()?;
            for idx in 0..sample_count {
                let genotype = genotypes.get(idx);
                for geno in genotype.iter() {
                    if let Some(x) = geno.index() {
                        geno_count.count[x as usize][idx] += 1; // ref alt1 alt2 ...
                    } else {
                        geno_count.count[allele_count][idx] += 1; // calc miss
                    }
                }
            }
            // fmt output
            print!(
                "{}\t{}\t{}\t{}\t{:.4}\t",
                String::from_utf8(row.header().rid2name(row.rid().unwrap()).unwrap().to_vec())
                    .unwrap(),
                pos,
                row.qual(),
                row.allele_count(),
                geno_count.miss_rate(allele_count)
            );
            for n in 0..allele_count {
                if n == 0 {
                    print!("{}:{:.4}\t", alleles[n], geno_count.ref_af(allele_count));
                } else {
                    if allele_count == 2 {
                        print!("{}:{:.4}", alleles[n],geno_count.alts_af(allele_count)[n - 1]);    
                    } else {
                        print!("{}:{:.4},", alleles[n],geno_count.alts_af(allele_count)[n - 1]);
                    }
                }
            }
            println!();
        }
    }
    Ok(())
}
