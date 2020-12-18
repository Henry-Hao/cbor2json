use serde_json::Serializer;
use serde_cbor::Deserializer;
use std::io::{BufWriter, BufReader, Write};
use std::fs::File;
use serde_transcode;
use clap::{App, Arg, crate_name, crate_authors, crate_version, crate_description};
fn main() {
    let app = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("input")
             .help("The input CBOR file")
             .required(true)
             .value_name("INPUT")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .help("The output json file")
             .value_name("OUTPUT")
             .required(true)
             .takes_value(true))
        .get_matches();
    let input = app.value_of("input").unwrap();
    let output = app.value_of("output").unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let writer = BufWriter::new(File::create(output).unwrap());


    let mut de = Deserializer::from_reader(reader);
    let mut se = Serializer::pretty(writer);
    serde_transcode::transcode(&mut de, &mut se).unwrap();
    se.into_inner().flush().unwrap()
}
