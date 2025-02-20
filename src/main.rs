use clap::Parser;
use csv;
use jsonseq;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

#[derive(Parser)]
struct Args {
    /// QLOG file path.
    qlog_file: String,

    /// Comma-separated path to get the data of interest.
    pattern: Pattern,

    #[clap(short = 'c', long = "csv", default_value = "output.csv")]
    /// Output CSV name.
    csv_name: String,
    // ... More arguments will come, e.g., for multi-thread.
}

#[derive(Debug, Clone)]
struct Pattern(Vec<String>);

impl FromStr for Pattern {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern(
            s.split(",").map(|i| i.to_string()).collect::<Vec<_>>(),
        ))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let in_file = File::open(&args.qlog_file)?;
    let json_reader = jsonseq::JsonSeqReader::new(in_file);

    // Open the CSV name to avoid computing everything for nothing.
    let out_file = File::create(&args.csv_name)?;
    let mut csv_wrt = csv::Writer::from_writer(out_file);

    // Write header using the last pattern.
    // Hope that this last pattern will be "precise" enough.
    csv_wrt.serialize(("time", args.pattern.0.last().unwrap_or(&"".to_string())))?;

    for elem in json_reader {
        let value = elem?;

        if let Some(v) = get_item(&value, &args.pattern) {
            csv_wrt.serialize(v)?;
        }
    }

    Ok(())
}

fn get_item<'a>(value: &'a Value, pattern: &'a Pattern) -> Option<(f64, &'a Value)> {
    // First element is always an object.
    let map = value.as_object()?;

    // We always record the time of the event.
    // If there is no "time", it may be the header.
    let time = map.get("time")?.as_f64()?;

    // Then we iterate on the requested value.
    let value = pattern
        .0
        .iter()
        .try_fold(value, |v, p| v.as_object().and_then(|m| m.get(p)))?;

    Some((time, value))
}
