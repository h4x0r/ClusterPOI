use anyhow::{Context, Result};
use clap::Parser;
use csv::{ReaderBuilder, WriterBuilder};
use linfa::{Dataset, prelude::Transformer};
use linfa_clustering::Dbscan;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input CSV file with lat/long data
    input: PathBuf,

    /// Output CSV file with cluster information
    #[arg(short, long)]
    output: PathBuf,

    /// Maximum distance between points in a cluster (in kilometers)
    #[arg(long, default_value = "1.0")]
    epsilon: f64,

    /// Minimum number of points required to form a cluster
    #[arg(long, default_value = "5")]
    min_samples: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Location {
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct LocationInput {
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Reading CSV file: {:?}", cli.input);
    let locations = read_csv(&cli.input)?;
    
    println!("Found {} locations", locations.len());
    
    if locations.is_empty() {
        anyhow::bail!("No locations found in the input file");
    }

    println!("Running DBSCAN clustering...");
    let clusters = perform_clustering(&locations, cli.epsilon, cli.min_samples)?;
    
    println!("Writing results to: {:?}", cli.output);
    write_csv(&cli.output, &locations, &clusters)?;
    
    let cluster_count = clusters.iter().max().unwrap_or(&-1) + 1;
    let noise_count = clusters.iter().filter(|&&c| c == -1).count();
    
    println!("Clustering complete!");
    println!("Found {} clusters", cluster_count);
    println!("{} points classified as noise", noise_count);

    Ok(())
}

fn read_csv(input_path: &PathBuf) -> Result<Vec<(f64, f64, HashMap<String, String>)>> {
    let file = File::open(input_path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    
    let mut locations = Vec::new();
    let headers = reader.headers()?.clone();
    
    let lat_idx = find_coordinate_column(&headers, &["lat", "latitude", "Latitude", "LAT"])?;
    let lon_idx = find_coordinate_column(&headers, &["lon", "lng", "long", "longitude", "Longitude", "LON", "LNG"])?;
    
    for result in reader.records() {
        let record = result?;
        
        let lat: f64 = record.get(lat_idx)
            .context("Failed to get latitude")?
            .parse()
            .context("Failed to parse latitude as float")?;
            
        let lon: f64 = record.get(lon_idx)
            .context("Failed to get longitude")?
            .parse()
            .context("Failed to parse longitude as float")?;
        
        let mut extra = HashMap::new();
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                extra.insert(header.to_string(), field.to_string());
            }
        }
        
        locations.push((lat, lon, extra));
    }
    
    Ok(locations)
}

fn find_coordinate_column(headers: &csv::StringRecord, possible_names: &[&str]) -> Result<usize> {
    for (i, header) in headers.iter().enumerate() {
        if possible_names.iter().any(|&name| header.eq_ignore_ascii_case(name)) {
            return Ok(i);
        }
    }
    
    anyhow::bail!(
        "Could not find coordinate column. Looking for one of: {:?}. Available headers: {:?}",
        possible_names,
        headers.iter().collect::<Vec<_>>()
    );
}

fn perform_clustering(
    locations: &[(f64, f64, HashMap<String, String>)], 
    epsilon: f64, 
    min_samples: usize
) -> Result<Vec<i32>> {
    if locations.len() < 2 {
        return Ok(vec![-1; locations.len()]);
    }

    let points: Array2<f64> = Array2::from_shape_vec(
        (locations.len(), 2),
        locations
            .iter()
            .flat_map(|(lat, lon, _)| vec![*lat, *lon])
            .collect(),
    )?;

    let dataset = Dataset::new(points, Array1::<usize>::zeros(locations.len()));
    
    let clusters = Dbscan::params(min_samples)
        .tolerance(epsilon / 111.0) // Convert km to approximate degrees
        .transform(dataset)?;
    
    Ok(clusters.targets().iter()
        .map(|&cluster| cluster.map(|c| c as i32).unwrap_or(-1))
        .collect())
}

fn write_csv(
    output_path: &PathBuf,
    locations: &[(f64, f64, HashMap<String, String>)],
    clusters: &[i32],
) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = WriterBuilder::new().from_writer(file);
    
    if locations.is_empty() {
        return Ok(());
    }

    let mut all_headers: Vec<String> = locations[0].2.keys().cloned().collect();
    all_headers.sort();
    all_headers.push("cluster".to_string());
    
    writer.write_record(&all_headers)?;
    
    for ((_lat, _lon, extra), &cluster) in locations.iter().zip(clusters.iter()) {
        let mut record = Vec::new();
        
        for header in &all_headers[..all_headers.len()-1] {
            record.push(extra.get(header).unwrap_or(&String::new()).clone());
        }
        
        record.push(cluster.to_string());
        writer.write_record(&record)?;
    }
    
    writer.flush()?;
    Ok(())
}
