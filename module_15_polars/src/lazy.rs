use std::time::Instant;
use polars::prelude::*;

const FILENAME: &str = "albums50M.csv";

fn main() {
    let time = Instant::now();
    // read csv file.
    let lf = LazyCsvReader::new(FILENAME.into())
        .with_has_header(true)
        .finish().unwrap();

    // Compute average rating per band and album.
    let query = lf.group_by([col("band"), col("album")])
        .agg([
            col("rating").mean().alias("mean"),
        ])
        .filter(col("album").eq(lit("Ashen")));

    println!("{}", query.explain(false).unwrap());
    println!("{}", query.explain(true).unwrap());

    let result = query.collect().unwrap();
    println!("{}", result);
    println!("{:?}", time.elapsed());
}
