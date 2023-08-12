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

impl StateTransition<Event, (), Infallible> for State {
    fn next(self, previous_event: Event) -> Result<Option<Self>, Infallible> {
        let state = match (self, previous_event) {
            (State::Red { .. }, Event::RedDone) => Self::Yellow { wait: 4 },
            (State::Yellow { .. }, Event::YellowDone) => Self::Green { wait: 5 },
            (State::Green { .. }, Event::GreenDone) => Self::Red { wait: 3 },
            (_, _) => {
                unreachable!()
            }
        };
        Ok(Some(state))
    }

    fn run(&mut self, _data: &mut ()) -> Event {
        match self {
            State::Red { wait } => {
                while *wait != 0 {
                    println!("Red");
                    std::thread::sleep(Duration::from_secs(1));
                    *wait -= 1;
                }
                Event::RedDone
            }
            State::Yellow { wait } => {
                while *wait != 0 {
                    println!("Yellow");
                    std::thread::sleep(Duration::from_secs(1));
                    *wait -= 1;
                }
                Event::YellowDone
            }
            State::Green { wait } => {
                while *wait != 0 {
                    println!("Green");
                    std::thread::sleep(Duration::from_secs(1));
                    *wait -= 1;
                }
                Event::GreenDone
            }
        }
    }
}

fn main() {
    StateMachine::run(State::Red { wait: 3 }, ());
}
