use clap::Parser;
use filter::Filter;
use filter::Pattern;
use serde_json::Value;
use std::error::Error;
use std::fs::File;

#[derive(Parser)]
struct Args {
    /// QLOG file path.
    qlog_file: String,

    /// Comma-separated path to get the data of interest.
    pattern: Pattern,

    #[clap(short = 'c', long = "csv", default_value = "output.csv")]
    /// Output CSV name.
    csv_name: String,

    #[clap(short = 'f', long = "filter")]
    /// Optional filter field.
    /// Comma-separated path to get to the filter value, which is the last element of the pattern.
    filter: Option<Filter>,
    // ... More arguments will come, e.g., for multi-thread.
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

        if let Some(v) = get_item(&value, &args.pattern, args.filter.as_ref()) {
            csv_wrt.serialize(v)?;
        }
    }

    Ok(())
}

fn get_item<'a>(
    value: &'a Value,
    pattern: &'a Pattern,
    filter: Option<&'a Filter>,
) -> Option<(f64, &'a Value)> {
    // First element is always an object.
    let map = value.as_object()?;

    // We always record the time of the event.
    // If there is no "time", it may be the header.
    let time = map.get("time")?.as_f64()?;

    // Potentially filter the entry.
    if !filter.map(|f| f.filter(value)).unwrap_or(true) {
        return None;
    }

    // Then we iterate on the requested value.
    let value = pattern
        .0
        .iter()
        .try_fold(value, |v, p| v.as_object().and_then(|m| m.get(p)))?;

    Some((time, value))
}

mod filter;
