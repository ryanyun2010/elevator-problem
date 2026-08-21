
use rand::rng;
use rand_distr::{Distribution, Normal};
use crate::{ELEVATORS, LATE_AFTER_SECS, NUM_PER_FLOOR};
use crate::structs::{
    elevator::Elevator,
    simulation::Simulation,
    worker::Worker,
};

pub struct HyperSimulation {
    arrival_time_mean: f64,
    arrival_time_std_dev: f64,
    num_simulations: usize
}

impl HyperSimulation {
    pub fn new(arrival_time_mean: f64, arrival_time_std_dev: f64, num_simulations: usize) -> HyperSimulation {
        HyperSimulation {
            arrival_time_mean,
            arrival_time_std_dev,
            num_simulations
        }
    }
    pub fn run(&self) -> (f64, f64) {
        let arrival_time_dist = Normal::new(self.arrival_time_mean, self.arrival_time_std_dev).unwrap();

        let mut total_late = 0usize;
        let mut total_late_squared = 0usize;

        for _ in 0..self.num_simulations {
            let mut rng = rng();
            let elevators = vec![Elevator::new(); ELEVATORS];
            let mut workers = Vec::with_capacity(NUM_PER_FLOOR.iter().sum());
            for (floor, &num) in NUM_PER_FLOOR.iter().enumerate() {
                for _ in 0..num {
                    let arrival_time = arrival_time_dist.sample(&mut rng).round() as isize;
                    workers.push(Worker {
                        arrival_time,
                        target_floor: (floor as i32) + 1,
                        clock_in_time: None,
                    });
                }
            }

            let mut simulation = Simulation::new(workers, elevators);
            simulation.initialize();
            simulation.simulate_until_done();

            let late = simulation.workers.iter().filter(|worker| worker.clock_in_time.unwrap() > LATE_AFTER_SECS).count();
            total_late += late;
            total_late_squared += late * late;
        }

        let mean_late = total_late as f64 / self.num_simulations as f64;
        let variance = total_late_squared as f64 / self.num_simulations as f64 - mean_late.powi(2);
        let standard_deviation = variance.max(0.0).sqrt();

        (mean_late, standard_deviation)
    }
}
