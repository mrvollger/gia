use crate::{
    io::{build_reader, match_input, match_output},
    types::{Bed6, InputFormat},
};
use anyhow::Result;
use bedrs::{Coordinates, NamedInterval};
use bstr::ByteSlice;
use csv::ByteRecord;
use faiquery::{FastaIndex, IndexedFasta};

fn build_fasta_index(fasta: &str) -> Result<FastaIndex> {
    let index_path = format!("{}.fai", fasta);
    FastaIndex::from_filepath(&index_path)
}

fn get_fasta_bed3(bed: Option<String>, fasta: &str, output: Option<String>) -> Result<()> {
    let handle = match_input(bed)?;
    let fasta_index = build_fasta_index(fasta)?;
    let fasta = IndexedFasta::new(fasta_index, fasta)?;

    let mut csv_reader = build_reader(handle);
    let mut byterecord = ByteRecord::new();
    let mut output = match_output(output)?;

    while csv_reader.read_byte_record(&mut byterecord)? {
        let record: NamedInterval<&str, usize> = byterecord.deserialize(None)?;
        match fasta.query_buffer(record.chr(), record.start(), record.end()) {
            Ok(buffer) => {
                write!(
                    output,
                    ">{}:{}-{}\n",
                    record.chr(),
                    record.start(),
                    record.end()
                )?;
                for subseq in buffer.split_str("\n") {
                    output.write(subseq)?;
                }
                output.write(b"\n")?;
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

fn get_fasta_bed6(bed: Option<String>, fasta: &str, output: Option<String>) -> Result<()> {
    let handle = match_input(bed)?;
    let fasta_index = build_fasta_index(fasta)?;
    let fasta = IndexedFasta::new(fasta_index, fasta)?;

    let mut csv_reader = build_reader(handle);
    let mut byterecord = ByteRecord::new();
    let mut output = match_output(output)?;

    while csv_reader.read_byte_record(&mut byterecord)? {
        let record: Bed6 = byterecord.deserialize(None)?;
        match fasta.query_buffer(record.chr, record.start, record.end) {
            Ok(buffer) => {
                write!(
                    output,
                    ">{}:{}-{}::{}::{}::{}\n",
                    record.chr, record.start, record.end, record.name, record.score, record.strand,
                )?;
                for subseq in buffer.split_str("\n") {
                    output.write(subseq)?;
                }
                output.write(b"\n")?;
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

pub fn get_fasta(
    bed: Option<String>,
    fasta: &str,
    output: Option<String>,
    format: InputFormat,
) -> Result<()> {
    match format {
        InputFormat::Bed3 => get_fasta_bed3(bed, fasta, output),
        InputFormat::Bed6 => get_fasta_bed6(bed, fasta, output),
    }
}
