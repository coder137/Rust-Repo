use std::{cell::RefCell, rc::Rc, time::Duration};

use behaviortree::{AsyncBehaviorTree, Behavior};

#[derive(Debug, Clone)]
enum Action {
    Red { wait: u64 },
    Yellow { wait: u64 },
    Green { wait: u64 },
}

#[tokio::main]
async fn main() {
    let traffic_light_sequence = Behavior::Sequence(vec![
        Behavior::Action(Action::Red { wait: 3 }),
        Behavior::Action(Action::Yellow { wait: 4 }),
        Behavior::Action(Action::Green { wait: 5 }),
    ]);

    let sequence = Behavior::While(
        Box::new(Behavior::WaitForever),
        vec![traffic_light_sequence],
    );

    let result = AsyncBehaviorTree::run(
        &sequence,
        Rc::default(),
        Rc::new(|action, _shared: Rc<RefCell<()>>| async move {
            let wait = match action {
                Action::Red { wait } => wait,
                Action::Yellow { wait } => wait,
                Action::Green { wait } => wait,
            };
            tokio::time::sleep(Duration::from_secs(wait)).await;
            Ok(())
        }),
    )
    .await;
    println!("Result: {:?}", result);
}
