mod structs;
use structs::{
    hyper_simulation::HyperSimulation
};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::BufWriter;
use csv::Writer;
use rayon::prelude::*;


const NUM_PER_FLOOR: [usize; 6] = [100,120,60,120,80,20];
const ELEVATORS: usize = 4;
const LATE_AFTER_SECS: isize = 5400; // start at 7:30

const ELEVATOR_CAPACITY: usize = 10;
const ELEVATOR_GROUND_FILL_TIME: i32 = 15;
const ELEVATOR_FLOOR_OPEN_TIME: i32 = 10;
const ELEVATOR_TRAVEL_TIME: i32 = 5;

const TARGET_MEAN: f64 = 110.0;

#[derive(Clone, Copy)]
struct SearchPoint {
    arrival_mean: usize,
    arrival_std_dev: usize,
}

#[derive(Clone, Copy)]
struct SearchResult {
    arrival_mean: usize,
    arrival_std_dev: usize,
    difference: f64
}

fn main() {
    let file = File::create("results.csv").expect("Failed to create results.csv");

    let writer = BufWriter::new(file);
    let mut csv_writer = Writer::from_writer(writer);

    csv_writer
        .write_record(["arrival_time_mean", "arrival_time_std_dev", "difference_to_target"])
        .expect("Failed to write CSV header");

    let mut search_points = Vec::with_capacity((539 - 360 + 1) * 180);
    for arrival_mean in 360..=539 {
        for arrival_std_dev in 1..=180 {
            search_points.push(SearchPoint {
                arrival_mean,
                arrival_std_dev,
            });
        }
    }

    let progress_bar = ProgressBar::new(search_points.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent:>3}% {msg}")
        .expect("Failed to create progress bar style")
        .progress_chars("=>-"),
    );

    let results: Vec<SearchResult> = search_points
        .par_iter()
        .map(|point| {
            let hyper_simulation = HyperSimulation::new(
                point.arrival_mean as f64 * 10.0,
                point.arrival_std_dev as f64 * 10.0,
                100,
            );

            let (mean_late, _) = hyper_simulation.run();
            let difference = (mean_late - TARGET_MEAN).abs();

            progress_bar.inc(1);

            SearchResult {
                arrival_mean: point.arrival_mean,
                arrival_std_dev: point.arrival_std_dev,
                difference
            }
        })
        .collect();

    progress_bar.finish_with_message("sweep complete");

    for result in results {
        csv_writer
            .write_record([
                result.arrival_mean.to_string(),
                result.arrival_std_dev.to_string(),
                result.difference.to_string(),
            ])
            .expect("Failed to write CSV record");
    }

    csv_writer.flush().expect("Failed to flush CSV writer");

    println!("Finished. Results written to results.csv");
}