use crate::{ELEVATOR_GROUND_FILL_TIME, ELEVATOR_FLOOR_OPEN_TIME, ELEVATOR_TRAVEL_TIME};
#[derive(Clone, Debug)]
pub struct Elevator {
    pub cur_workers: Vec<usize>, // indices of workers in the elevator
    pub cur_floor: i32,
    pub state: ElevatorState,
    pub time_left_in_action: i32,
}

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub enum ElevatorState {
    GroundFill,
    GoingUp, 
    GoingDown, 
    FloorOpen 
}

impl ElevatorState {
    pub fn duration(&self) -> i32 {
        match self {
            ElevatorState::GroundFill => ELEVATOR_GROUND_FILL_TIME,
            ElevatorState::GoingUp | ElevatorState::GoingDown => ELEVATOR_TRAVEL_TIME,
            ElevatorState::FloorOpen => ELEVATOR_FLOOR_OPEN_TIME,
        }
    }
}

impl Elevator {
    pub fn new() -> Elevator {
        Elevator {
            cur_workers: Vec::new(),
            cur_floor: 0,
            time_left_in_action: ELEVATOR_GROUND_FILL_TIME,
            state: ElevatorState::GroundFill,
        }
    }
    pub fn set_state(&mut self, state: ElevatorState) {
        self.state = state;
        self.time_left_in_action = state.duration();
    }
}