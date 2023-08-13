use std::{convert::Infallible, time::Duration};

use async_trait::async_trait;
use statemachine::{AsyncStateMachine, AsyncStateTransition};
use tokio::{select, signal};

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

#[async_trait]
impl AsyncStateTransition<Event, Common, Infallible> for TrafficLight {
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

    async fn run(&mut self, common: &mut Common) -> Event {
        match self.current_state {
            State::Red { wait } => {
                common.wait_for("Red", wait).await;
                Event::RedDone
            }
            State::Yellow { wait } => {
                common.wait_for("Yellow", wait).await;
                Event::YellowDone
            }
            State::Green { wait } => {
                common.wait_for("Green", wait).await;
                Event::GreenDone
            }
        }
    }
}

struct Common;

impl Common {
    async fn wait_for(&self, identifier: &str, mut time: usize) {
        while time != 0 {
            println!("{identifier}");
            tokio::time::sleep(Duration::from_secs(1)).await;
            time -= 1;
        }
    }
}

#[tokio::main]
async fn main() {
    select! {
        result = AsyncStateMachine::run(TrafficLight::red_light(), Common) => {
            println!("Result: {result:?}");
        }
        _ = signal::ctrl_c() => {
            println!("Exiting");
        }
    };
}
