use async_trait::async_trait;

#[async_trait]
pub trait AsyncStateTransition<Event, Shared, Error>
where
    Self: Sized,
{
    // Required
    fn next(self, previous_event: Event) -> Result<Option<Self>, Error>;
    async fn run(&mut self, data: &mut Shared) -> Event;

    // Optional overrides
    async fn start(&mut self, _data: &mut Shared) {}
    async fn end(&mut self, _data: &mut Shared) {}
}

pub struct AsyncStateMachine;

impl AsyncStateMachine {
    pub async fn run<Event, Shared, Error>(
        // * Investigate, This does not compile without std::marker::Send (limitation of async_trait crate?)
        mut state: impl AsyncStateTransition<Event, Shared, Error> + std::marker::Send,
        mut shared: Shared,
    ) -> Result<(), Error> {
        loop {
            state.start(&mut shared).await;
            let event = state.run(&mut shared).await;
            state.end(&mut shared).await;
            let maybe_state = state.next(event)?;
            match maybe_state {
                Some(s) => {
                    state = s;
                }
                None => {
                    break;
                }
            }
        }
        Ok(())
    }
}
