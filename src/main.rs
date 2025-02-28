use clap::Parser;
use filter::convert_value_and_get;
use filter::Filter;
use filter::Pattern;
use serde_json::Value;
use std::error::Error;
use std::fs::File;

#[derive(Parser)]
struct Args {
    /// QLOG file path.
    qlog_file: String,

    #[clap(short = 'p', long = "pattern", num_args = 1.., value_delimiter = ',')]
    /// Slash-separated path to get the data of interest.
    /// Multiple values can be requested.
    pattern: Vec<Pattern>,

    #[clap(short = 'c', long = "csv", default_value = "output.csv")]
    /// Output CSV name.
    csv_name: String,

    #[clap(short = 'f', long = "filter", num_args = 1.., value_delimiter = ',')]
    /// Optional filter field.
    /// Slash-separated path to get to the filter value, which is the last element of the pattern.
    filter: Option<Vec<Filter>>,
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
    let mut header = args
        .pattern
        .iter()
        .map(|p| p.0.last().map_or("", |v| v))
        .collect::<Vec<_>>();
    header.insert(0, "time");
    csv_wrt.serialize(&header)?;

    for elem in json_reader {
        let value = elem?;

        if let Some(v) = get_item(&value, &args.pattern, args.filter.as_deref()) {
            csv_wrt.serialize(v)?;
        }
    }

    Ok(())
}

fn get_item<'a>(
    value: &'a Value,
    patterns: &'a [Pattern],
    filter: Option<&'a [Filter]>,
) -> Option<(f64, Vec<&'a Value>)> {
    // First element is always an object.
    let map = value.as_object()?;

    // We always record the time of the event.
    // If there is no "time", it may be the header.
    let time = map.get("time")?.as_f64()?;

    // Potentially filter the entry.
    if !filter
        .map(|filters| filters.iter().all(|f| f.filter(value)))
        .unwrap_or(true)
    {
        return None;
    }

    // Then we iterate on the requested value.
    let values = patterns
        .iter()
        .map(|pattern| {
            pattern
                .0
                .iter()
                .try_fold(value, |v, p| convert_value_and_get(v, p))
        })
        .collect::<Option<Vec<_>>>()?;

    Some((time, values))
}

mod filter;
