use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Extends the intervals of a BED file
    ///
    /// The extension is either done on both sides at once
    /// or on the left and right side separately
    Extend {
        /// Input BED file to extend (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Extend intervals on both sides by the same provided amount
        #[clap(short, long, required_unless_present_any(["left", "right"]), conflicts_with_all(&["left", "right"]))]
        both: Option<usize>,

        /// Extend intervals on the left side by the provided amount
        #[clap(short, long, required_unless_present_any(["both", "right"]))]
        left: Option<usize>,

        /// Extend intervals on the right side by the provided amount
        #[clap(short, long, required_unless_present_any(["both", "left"]))]
        right: Option<usize>,
    },

    /// Intersects two BED files
    Intersect {
        #[clap(short, long)]
        a: Option<String>,

        #[clap(short, long)]
        b: String,

        #[clap(short, long)]
        output: Option<String>,
    },

    /// Merges intervals of a BED file with overlapping regions
    Merge {
        /// Input BED file to merge (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Assume input is sorted (default=false)
        #[clap(short, long)]
        sorted: bool,
    },

    /// Sorts a BED file by chromosome, start, and end
    Sort {
        /// Input GIA file to sort (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output GIA file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Extracts FASTA sequences using intervals from a BED file
    GetFasta {
        /// BED file containing intervals to extract
        #[clap(short, long)]
        bed: Option<String>,

        /// FASTA file to extract sequences from (assumes <fasta>.fai exists)
        #[clap(short, long)]
        fasta: String,

        /// Output FASTA file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Name map file to use (in case chromosome names are non-integers)
        #[clap(short, long)]
        map: Option<String>,

        /// Number of threads to use (use zero for all available cores)
        #[clap(short, long, default_value = "1")]
        threads: Option<usize>,
    },

    /// Builds a two column map of chromosome names to integers
    /// and writes the map and BED file with integer chromosome names
    /// to disk
    ///
    /// The map file is a two column file with the first column
    /// containing the integer chromosome index and the second column
    /// containing the original chromosome name
    NameMap {
        /// Input BED file to map chromosome names (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Output map file to write to (default=name_map.tsv)
        #[clap(short, long)]
        map: Option<String>,
    },
}
