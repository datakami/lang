use std::error::Error;
use std::io::{self};

use csv::{ReaderBuilder, WriterBuilder, Reader, Writer};
use serde::Deserialize;
use whichlang::Lang;

#[derive(Debug, Deserialize)]
struct Record {
    index: u32,
    text: String,
}


fn detect_language(text: &str) -> Result<String, Box<dyn Error>> {
    let detected: Lang = whichlang::detect_language(text);
    Ok(detected.three_letter_code().to_string())
}

fn process_csv(rdr: &mut Reader<io::Stdin>, wrt: &mut Writer<io::Stdout>) -> Result<(), Box<dyn Error>> {
    for result in rdr.deserialize() {
        let record: Record = result?;
        let language = detect_language(&record.text)?;


        wrt.write_record(&[record.index.to_string(), language])?;
    }

    Ok(())
}

fn main() {
    let mut rdr = ReaderBuilder::new().from_reader(io::stdin());
    let mut wrt = WriterBuilder::new().from_writer(io::stdout());
    match process_csv(&mut rdr, &mut wrt) {
        Ok(()) => eprintln!("Processing completed successfully."),
        Err(err) => eprintln!("An error occurred: {}", err),
    }
}
