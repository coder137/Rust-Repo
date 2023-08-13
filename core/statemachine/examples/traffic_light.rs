use std::{convert::Infallible, time::Duration};

use statemachine::{StateMachine, StateTransition};

#[derive(Debug)]
enum State {
    Red { wait: usize },
    Yellow { wait: usize },
    Green { wait: usize },
}

#[derive(Debug)]
enum Event {
    RedDone,
    YellowDone,
    GreenDone,
}

struct TrafficLight {
    current_state: State,
}

impl TrafficLight {
    fn red_light() -> Self {
        Self {
            current_state: State::Red { wait: 3 },
        }
    }

    fn yellow_light() -> Self {
        Self {
            current_state: State::Yellow { wait: 4 },
        }
    }

    fn green_light() -> Self {
        Self {
            current_state: State::Green { wait: 5 },
        }
    }
}

impl StateTransition<Event, Common, Infallible> for TrafficLight {
    fn next(self, previous_event: Event) -> Result<Option<Self>, Infallible> {
        let state = match (self.current_state, previous_event) {
            (State::Red { .. }, Event::RedDone) => Self::yellow_light(),
            (State::Yellow { .. }, Event::YellowDone) => Self::green_light(),
            (State::Green { .. }, Event::GreenDone) => Self::red_light(),
            (_, _) => {
                unreachable!()
            }
        };
        Ok(Some(state))
    }

    fn run(&mut self, common: &mut Common) -> Event {
        match self.current_state {
            State::Red { wait } => {
                common.wait_for("Red", wait);
                Event::RedDone
            }
            State::Yellow { wait } => {
                common.wait_for("Yellow", wait);
                Event::YellowDone
            }
            State::Green { wait } => {
                common.wait_for("Green", wait);
                Event::GreenDone
            }
        }
    }

    fn start(&mut self, _data: &mut Common) {
        println!("Start");
    }

    fn end(&mut self, _data: &mut Common) {
        println!("End");
    }
}

struct Common;

impl Common {
    fn wait_for(&self, identifier: &str, mut time: usize) {
        while time != 0 {
            println!("{identifier}");
            std::thread::sleep(Duration::from_secs(1));
            time -= 1;
        }
    }
}

fn main() {
    StateMachine::run(TrafficLight::red_light(), Common);
}
