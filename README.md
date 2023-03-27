# vcfkit
🦀 a simple program for vcf/bcf file manipulation

## install

```bash
git clone https://github.com/sharkLoc/vcfkit.git
cd vcfkit
cargo b --release
# mv target/release/vcfkit to anywhere you want 
```

## usage

```bash
vcfkit --help
vcfkit: a simple program for vcf/bcf file manipulation
repo: https::github.com/sharkLoc/vcfkit.git
email: dwood8146@gmail.com

Usage: vcfkit <COMMAND>

Commands:
  concat
          concatenate VCF/BCF files from the same set of samples
  convert
          convert BCF file to VCF file formats
  select
          select SNP/INDEL variants from a VCF file
  split
          split vcf file by chromosome name
  stats
          statistics including alleles frequence, missing rate, etc
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
** any bugs please report issues **
