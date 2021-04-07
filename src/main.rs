use std::error::Error;
use std::time::Instant;

use serde::Deserialize;
use simsearch::SimSearch;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CitiesRecordRaw {
    geonameid: usize,
    name: String,
    asciiname: String,
    alternatenames: String,
    latitude: f64,
    longitude: f64,
    feature_class: String,
    feature_code: String,
    country_code: String,
    cc2: String,
    admin1_code: String,
    admin2_code: String,
    admin3_code: String,
    admin4_code: String,
    population: String,
    elevation: String,
    dem: String,
    timezone: String,
    modification_date: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path("misc/cities500.txt")?;

    let records = rdr
        .deserialize()
        .into_iter()
        .map(|row| {
            let record: CitiesRecordRaw = row.unwrap();
            record
        })
        .collect::<Vec<CitiesRecordRaw>>();

    let mut records_map: HashMap<usize, CitiesRecordRaw> = HashMap::new();
    let mut engine: SimSearch<usize> = SimSearch::new();

    let now = Instant::now();
    for item in records {
        engine.insert(item.geonameid, &item.name);
        records_map.insert(item.geonameid, item);
    }
    println!(
        "Indexing of {} records took {}ms",
        records_map.len(),
        now.elapsed().as_millis(),
    );
    let city = "Voronezh";
    let result = engine.search(city);
    println!(
        "Search city: {} found: {} records. Top 10:",
        city,
        result.len()
    );
    for item in result.iter().take(10) {
        let record = records_map.get(&item).unwrap();
        println!(
            "{} {} {}",
            record.name, record.country_code, record.geonameid,
        );
    }

    Ok(())
}
