pub trait StateTransition<Event, Shared, Error>
where
    Self: Sized,
{
    fn next(self, previous_event: Event) -> Result<Option<Self>, Error>;
    fn run(&mut self, data: &mut Shared) -> Event;
}

pub struct StateMachine;

impl StateMachine {
    pub fn run<Event, Shared, Error>(
        mut state: impl StateTransition<Event, Shared, Error>,
        mut shared: Shared,
    ) {
        loop {
            let event = state.run(&mut shared);
            let maybe_state = state.next(event);
            match maybe_state {
                Ok(Some(s)) => {
                    state = s;
                }
                Ok(None) => {
                    break;
                }
                Err(_e) => {
                    // error!("Error: {:?}", e);
                    break;
                }
            }
        }
    }
}
