pub trait StateTransition<Event, Shared, Error>
where
    Self: Sized,
{
    // Required
    fn next(self, previous_event: Event) -> Result<Option<Self>, Error>;
    fn run(&mut self, data: &mut Shared) -> Event;

    // Optional overrides
    fn start(&mut self, _data: &mut Shared) {}
    fn end(&mut self, _data: &mut Shared) {}
}

pub struct StateMachine;

impl StateMachine {
    pub fn run<Event, Shared, Error>(
        mut state: impl StateTransition<Event, Shared, Error>,
        mut shared: Shared,
    ) {
        loop {
            state.start(&mut shared);
            let event = state.run(&mut shared);
            state.end(&mut shared);
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
