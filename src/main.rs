mod structs;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use structs::{
    hyper_simulation::HyperSimulation
};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::BufWriter;
use csv::Writer;


const NUM_PER_FLOOR: [usize; 6] = [100,120,60,120,80,20];
const ELEVATORS: usize = 4;
const LATE_AFTER_SECS: isize = 5400; // start at 7:30

const ELEVATOR_CAPACITY: usize = 10;
const ELEVATOR_GROUND_FILL_TIME: i32 = 15;
const ELEVATOR_FLOOR_OPEN_TIME: i32 = 10;
const ELEVATOR_TRAVEL_TIME: i32 = 5;

const TARGET_MEAN: f64 = 110.0;
const NUM_TRIALS: usize = 1_000_000;

// fn main() {
//     let hyper_simulation = HyperSimulation::new(
//         4760.0, // 8:49
//         300.0, // 5 min std deviation
//         10000,
//         None,
//     );
//     let d = hyper_simulation.run();
//     println!("No assignment: mean late {:.2}, std dev {:.2}", d.0, d.1);
//     let hyper_sim_with_assignment = HyperSimulation::new(
//         4760.0,
//         300.0,
//         10000,
//         Some(structs::hyper_simulation::ElevatorAssignment {
//             floor_1_assigned: vec![0, 8, 20, 72], // 100
//             floor_2_assigned: vec![0, 10, 110, 0],
//             floor_3_assigned: vec![0, 13, 17, 30],
//             floor_4_assigned: vec![0, 120, 0, 0],
//             floor_5_assigned: vec![80, 0, 0, 0],
//             floor_6_assigned: vec![20, 0, 0, 0],
//         }),
//     );
//     let d2 = hyper_sim_with_assignment.run();
//     println!("With assignment: mean late {:.2}, std dev {:.2}", d2.0, d2.1);
// }

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
                50, None
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

// use rand::Rng;
// use rand::RngExt;

// use crate::structs::hyper_simulation::ElevatorAssignment;

// fn random_floor_assignment(total: usize, rng: &mut impl Rng) -> Vec<usize> {
//     let units = total / 10;

//     let a = rng.random_range(0..=units);
//     let b = rng.random_range(0..=(units - a));
//     let c = rng.random_range(0..=(units - a - b));
//     let d = units - a - b - c;

//     return vec![a * 10, b * 10, c * 10, d * 10]
// }

// fn canonicalize(assignment: &mut ElevatorAssignment) {
//     let mut elevators: Vec<Vec<usize>> = (0..4)
//         .map(|e| {
//             vec![
//                 assignment.floor_1_assigned[e],
//                 assignment.floor_2_assigned[e],
//                 assignment.floor_3_assigned[e],
//                 assignment.floor_4_assigned[e],
//                 assignment.floor_5_assigned[e],
//                 assignment.floor_6_assigned[e],
//             ]
//         })
//         .collect();

//     // Lexicographic ordering of the elevator assignment vectors.
//     elevators.sort();

//     for e in 0..4 {
//         assignment.floor_1_assigned[e] = elevators[e][0];
//         assignment.floor_2_assigned[e] = elevators[e][1];
//         assignment.floor_3_assigned[e] = elevators[e][2];
//         assignment.floor_4_assigned[e] = elevators[e][3];
//         assignment.floor_5_assigned[e] = elevators[e][4];
//         assignment.floor_6_assigned[e] = elevators[e][5];
//     }
// }

// fn random_assignment(rng: &mut impl Rng) -> ElevatorAssignment {
//     let floors: Vec<Vec<usize>> = NUM_PER_FLOOR
//         .iter()
//         .map(|&total| random_floor_assignment(total, rng))
//         .collect();

//     let mut assignment = ElevatorAssignment {
//         floor_1_assigned: floors[0].clone(),
//         floor_2_assigned: floors[1].clone(),
//         floor_3_assigned: floors[2].clone(),
//         floor_4_assigned: floors[3].clone(),
//         floor_5_assigned: floors[4].clone(),
//         floor_6_assigned: floors[5].clone(),
//     };

//     canonicalize(&mut assignment);

//     assignment
// }

// fn main() {
//     let no_assignment_simulation = HyperSimulation::new(
//         4760.0,
//         300.0,
//         100,
//         None,
//     );
//     let d = no_assignment_simulation.run();
//     println!("No assignment: mean late {:.2}, std dev {:.2}", d.0, d.1);
//     let mut rng = rand::rng();

//     let progress = ProgressBar::new(NUM_TRIALS as u64);

//     progress.set_style(
//         ProgressStyle::with_template(
//             "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | best: {msg}"
//         )
//         .unwrap()
//         .progress_chars("##-"),
//     );

//     let mut best_mean = f64::INFINITY;
//     let mut best_std = f64::INFINITY;
//     let mut best_assignment: Option<ElevatorAssignment> = None;

//     for _ in 0..NUM_TRIALS {
//         let assignment = random_assignment(&mut rng);

//         let simulation = HyperSimulation::new(
//             4760.0,
//             300.0,
//             100,
//             Some(assignment.clone()),
//         );

//         let (mean_late, std_dev_late) = simulation.run();

//         if mean_late < best_mean {
//             best_mean = mean_late;
//             best_std = std_dev_late;
//             best_assignment = Some(assignment);

//             progress.set_message(format!(
//                 "{:.2} min (std {:.2})",
//                 best_mean,
//                 best_std
//             ));

//             println!(
//                 "\nNew best!\n\
//                  Mean late: {:.2}\n\
//                  Std dev:   {:.2}\n\
//                  Assignment: {:#?}\n",
//                 best_mean,
//                 best_std,
//                 best_assignment.as_ref().unwrap()
//             );
//         }

//         progress.inc(1);
//     }

//     progress.finish_with_message(format!(
//         "Best mean: {:.2}, std: {:.2}",
//         best_mean, best_std
//     ));

//     println!("\n FINAL BEST");
//     println!("Mean late: {:.2}", best_mean);
//     println!("Std dev:   {:.2}", best_std);
//     println!(
//         "Assignment:\n{:#?}",
//         best_assignment.unwrap()
//     );
// }

