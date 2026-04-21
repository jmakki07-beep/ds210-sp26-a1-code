use std::time::Instant;
use polars::prelude::*;

const FILENAME: &str = "albums50M.csv";

fn main() {
    let time = Instant::now();
    // read csv file.
    let reader = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(FILENAME.into()))
        .unwrap();

    // Read all file as a big dataframe.
    let df = reader.finish().unwrap();
    println!("{}", df.height());
    println!("{:?}", df.get_row(0).unwrap());

    // Compute average rating per band and album.
    let averages = df.group_by(["band", "album"]).unwrap()
        .select(["rating"]).mean().unwrap();
    let condition = averages.column("album").unwrap().str().unwrap().equal("Ashen");
    let result = averages.filter(&condition).unwrap();
    println!("{}", result);
    println!("{:?}", time.elapsed());
}
