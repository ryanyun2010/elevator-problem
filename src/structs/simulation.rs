use crate::structs::{
    elevator::{Elevator, ElevatorState},
    worker::Worker,
};
use crate::ELEVATOR_CAPACITY;

#[derive(Debug)]
pub struct Simulation {
    elevators: Vec<Elevator>,
    pub workers: Vec<Worker>,
    cur_time: isize, // seconds since 8:00, 3600 seconds = 9:00
    workers_waiting: Vec<usize>, // indices of workers waiting for an elevator
    workers_clocked_in: usize, // number of workers who have clocked in
    last_worker_arrived: Option<isize>, // time the last worker arrived, None if no workers have arrived yet
}

impl Simulation {
    pub fn new(workers: Vec<Worker>, elevators: Vec<Elevator>) -> Simulation {
        Simulation {
            elevators,
            workers,
            cur_time: 0,
            workers_waiting: Vec::new(),
            workers_clocked_in: 0,
            last_worker_arrived: None,
        }
    }

    pub fn initialize(&mut self) {
        self.cur_time = 0;
        for elevator in self.elevators.iter_mut() {
            elevator.set_state(ElevatorState::GroundFill);
            elevator.cur_floor = 0;
            elevator.cur_workers.clear();
        }

        self.workers_waiting.clear();
        self.workers.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
        for (i, worker) in self.workers.iter().enumerate() {
            if worker.arrival_time <= 0 {
                self.workers_waiting.push(i);
                self.last_worker_arrived = Some(worker.arrival_time);
            } else {
                break;
            }
        }
    }

    pub fn simulate_until_done(&mut self) {
        while self.workers_clocked_in < self.workers.len() {
            self.next();
        }
    }

    fn next(&mut self) {
        self.cur_time += 1;

        self.arrive_workers();
        self.update_elevators();
    }

    fn arrive_workers(&mut self) {
        for (i, worker) in self.workers.iter().skip(self.last_worker_arrived.unwrap_or(0) as usize).enumerate() {
            if worker.arrival_time == self.cur_time {
                self.workers_waiting.push(i);
                self.last_worker_arrived = Some(i as isize);
            } else{
                break;
            }
        }
    }

    fn update_elevators(&mut self) {
        for elevator in self.elevators.iter_mut() {
            if elevator.time_left_in_action > 0 {
                if elevator.state == ElevatorState::GroundFill { // if on ground, attempt to fill remaining space
                    let available_space = ELEVATOR_CAPACITY - elevator.cur_workers.len();
                    let count = available_space.min(self.workers_waiting.len());

                    elevator
                        .cur_workers
                        .extend(self.workers_waiting.drain(..count));
                }
                elevator.time_left_in_action -= 1;
            } else {
                match elevator.state {
                    ElevatorState::GroundFill => elevator.set_state( 
                        if elevator.cur_workers.is_empty() {ElevatorState::GroundFill} else {ElevatorState::GoingUp}
                    ),

                    ElevatorState::GoingUp => {
                        elevator.cur_floor += 1;
                        
                        let mut got_off = 0;
                        elevator.cur_workers.retain(
                            |worker_index| 
                            if self.workers[*worker_index].target_floor == elevator.cur_floor {
                                self.workers[*worker_index].clock_in_time = Some(self.cur_time);
                                got_off += 1;
                                false
                            } else {true}   
                        );

                        if got_off > 0 {
                            self.workers_clocked_in += got_off;
                            elevator.set_state(ElevatorState::FloorOpen);
                        } else {
                            let target_above = elevator.cur_workers.iter().any(
                                |&worker_index| self.workers[worker_index].target_floor > elevator.cur_floor);

                            elevator.set_state(
                                if target_above {ElevatorState::GoingUp} else {ElevatorState::GoingDown}
                            );
                        }
                    },

                    ElevatorState::FloorOpen => {
                        let target_above = elevator.cur_workers.iter().any(
                            |&worker_index| self.workers[worker_index].target_floor > elevator.cur_floor);

                        elevator.set_state(
                            if target_above {ElevatorState::GoingUp} else {ElevatorState::GoingDown}
                        );
                    },


                    ElevatorState::GoingDown => {
                        if elevator.cur_floor > 0 {
                            elevator.cur_floor -= 1;
                        }

                        elevator.set_state(
                            if elevator.cur_floor == 0 {ElevatorState::GroundFill} else {ElevatorState::GoingDown}
                        );
                    }
                }
            }
        }
    }
}