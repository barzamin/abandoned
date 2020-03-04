use osmpbfreader::{OsmObj, OsmPbfReader};
use regex::Regex;
use std::fs::File;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opts::from_args();
    let r = File::open(opt.input)?;
    let mut pbf = OsmPbfReader::new(r);
    let re = Regex::new(r"^abandoned:")?;

    let abandoned: Vec<OsmObj> = pbf
        .par_iter()
        .map(Result::unwrap)
        .filter(|obj| obj.tags().keys().any(|k| re.is_match(k)))
        .collect();

    println!("{}", ron::ser::to_string(&abandoned)?);

    Ok(())
}
