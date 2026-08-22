use crate::structs::{
    elevator::{Elevator, ElevatorState}, hyper_simulation::ElevatorAssignment, worker::Worker,
};
use crate::ELEVATOR_CAPACITY;

#[derive(Debug)]
pub struct Simulation {
    elevators: Vec<Elevator>,
    pub workers: Vec<Worker>,
    cur_time: isize, // seconds since 8:00, 3600 seconds = 9:00
    workers_waiting: Vec<usize>, // indices of workers waiting for an elevator
    workers_clocked_in: usize, // number of workers who have clocked in
    elevator_assignments: Option<ElevatorAssignment>, // indices of workers assigned to each elevator
}

impl Simulation {
    pub fn new(workers: Vec<Worker>, elevators: Vec<Elevator>, elevator_assignments: Option<ElevatorAssignment>) -> Simulation {
        Simulation {
            elevators,
            workers,
            cur_time: 0,
            workers_waiting: Vec::new(),
            workers_clocked_in: 0,
            elevator_assignments
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
        if self.elevator_assignments.is_some() {
            self.assign_elevators();
        }
        for (i, worker) in self.workers.iter().enumerate() {
            if worker.arrival_time <= 0 {
                self.workers_waiting.push(i);
            } else {
                break;
            }
        }
    }
    pub fn assign_elevators(&mut self) {
        let mut eve = self.elevator_assignments.as_ref().unwrap().clone();
        for worker in self.workers.iter_mut().enumerate() {
            let flooreve: &mut Vec<usize> = match worker.1.target_floor {
                1 => eve.floor_1_assigned.as_mut(),
                2 => eve.floor_2_assigned.as_mut(),
                3 => eve.floor_3_assigned.as_mut(),
                4 => eve.floor_4_assigned.as_mut(),
                5 => eve.floor_5_assigned.as_mut(),
                6 => eve.floor_6_assigned.as_mut(),
                _ => panic!("Invalid floor number"),
            };
            for (elevator_index, ppl) in flooreve.iter_mut().enumerate() {
                if *ppl > 0 {
                    worker.1.elevator_assignment = Some(elevator_index);
                    *ppl -= 1;
                    break;
                } else {
                    continue;
                }
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
        for (i, worker) in self.workers.iter().enumerate() {
            if worker.arrival_time < self.cur_time {
                continue;
            } else if worker.arrival_time == self.cur_time {
                self.workers_waiting.push(i);
            } else{
                break;
            }
        }
    }

    fn update_elevators(&mut self) {
        for (elevator_index, elevator) in self.elevators.iter_mut().enumerate() {
            if elevator.time_left_in_action > 0 {
                if elevator.state == ElevatorState::GroundFill { // if on ground, attempt to fill remaining space
                    let mut available_space = ELEVATOR_CAPACITY - elevator.cur_workers.len();
                    let count = available_space.min(self.workers_waiting.len());

                    elevator
                        .cur_workers
                        .extend(self.workers_waiting.extract_if(.., |n| {
                            if available_space == 0 {
                                return false;
                            }
                            let worker = &self.workers[*n];
                            if worker.elevator_assignment.is_none() || worker.elevator_assignment.unwrap() == elevator_index {
                                available_space -= 1;
                                true
                            } else {
                                false
                            }
                            
                        }).take(count));
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