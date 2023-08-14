use std::{
    cell::RefCell,
    future::{self, Future},
    rc::Rc,
    time::Duration,
};

use async_recursion::async_recursion;
use tokio::{task::LocalSet, sync::mpsc, select};
use tokio_util::sync::CancellationToken;

/// Describes a behavior.
///
/// This is used for more complex event logic.
/// Can also be used for game AI.
#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Debug)]
pub enum Behavior<A> {
    /// Waits an amount of time before continuing
    ///
    /// Duration
    Wait(Duration),
    /// Wait forever.
    WaitForever,
    /// A high level description of an action.
    ///
    /// An Action can either be "condition" which does not
    /// alter the system and returns either `Success` or `Failure`
    /// - e.g IsDoorOpen? IsNetworkDown?
    ///
    /// Or it can be an "act" that can alter the system
    /// and returns either `Success`, `Failure` or `Running`
    /// - e.g OpenDoor, NetworkShutdown
    Action(A),
    /// Converts `Success` into `Failure` and vice versa.
    Invert(Box<Behavior<A>>),
    /// Ignores failures and returns `Success`.
    AlwaysSucceed(Box<Behavior<A>>),
    /// Runs behaviors one by one until a behavior succeeds.
    ///
    /// If a behavior fails it will try the next one.
    /// Fails if the last behavior fails.
    /// Can be thought of as a short-circuited logical OR gate.
    Select(Vec<Behavior<A>>),
    /// `If(condition, success, failure)`
    If(Box<Behavior<A>>, Box<Behavior<A>>, Box<Behavior<A>>),
    /// Runs behaviors one by one until all succeeded.
    ///
    /// The sequence fails if a behavior fails.
    /// The sequence succeeds if all the behavior succeeds.
    /// Can be thought of as a short-circuited logical AND gate.
    Sequence(Vec<Behavior<A>>),
    /// Loops while conditional behavior is running.
    ///
    /// Succeeds if the conditional behavior succeeds.
    /// Fails if the conditional behavior fails,
    /// or if any behavior in the loop body fails.
    While(Box<Behavior<A>>, Vec<Behavior<A>>),
    // /// Runs all behaviors in parallel until all succeeded.
    // ///
    // /// Succeeds if all behaviors succeed.
    // /// Fails is any behavior fails.
    // WhenAll(Vec<Behavior<A>>),
    // /// Runs all behaviors in parallel until one succeeds.
    // ///
    // /// Succeeds if one behavior succeeds.
    // /// Fails if all behaviors failed.
    // WhenAny(Vec<Behavior<A>>),
    // /// Runs all behaviors in parallel until all succeeds in sequence.
    // ///
    // /// Succeeds if all behaviors succeed, but only if succeeding in sequence.
    // /// Fails if one behavior fails.
    // After(Vec<Behavior<A>>),
}

pub struct AsyncBehaviorTree;

impl AsyncBehaviorTree {
    ///
    #[async_recursion(?Send)]
    pub async fn run<A, Shared, F, Fut>(
        sequence: &Behavior<A>,
        shared: Rc<RefCell<Shared>>,
        f: Rc<F>,
    ) -> Result<(), ()>
    where
        A: std::fmt::Debug + Clone + 'static,
        Shared: 'static,
        Fut: Future<Output = Result<(), ()>>,
        F: Fn(A, Rc<RefCell<Shared>>) -> Fut + 'static,
    {
        println!("Sequence: {sequence:?}");
        match sequence {
            Behavior::Wait(duration) => {
                tokio::time::sleep(*duration).await;
                Ok(())
            }
            Behavior::WaitForever => {
                future::pending::<()>().await;
                Ok(())
            }
            Behavior::Action(a) => f(a.clone(), shared).await,
            Behavior::Invert(behavior) => match Self::run(behavior, shared, f).await {
                Ok(_) => Err(()),
                Err(_) => Ok(()),
            },
            Behavior::AlwaysSucceed(behavior) => {
                let _ = Self::run(behavior, shared, f).await;
                Ok(())
            }
            Behavior::Select(behaviors) => {
                let mut ret_result = Err(());
                for behavior in behaviors {
                    if Self::run(behavior, shared.clone(), f.clone()).await.is_ok() {
                        ret_result = Ok(());
                        break;
                    }
                }
                ret_result
            }
            Behavior::If(condition, success, failure) => {
                match Self::run(condition, shared.clone(), f.clone()).await {
                    Ok(_) => Self::run(success, shared, f).await,
                    Err(_) => Self::run(failure, shared, f).await,
                }
            }
            Behavior::Sequence(behaviors) => {
                let mut ret_result = Ok(());
                for behavior in behaviors {
                    if Self::run(behavior, shared.clone(), f.clone()).await.is_err() {
                        ret_result = Err(());
                        break;
                    }
                }
                ret_result
            }
            Behavior::While(conditional, behaviors) => {
                let local_set = LocalSet::default();
                let (tx, mut rx) = mpsc::channel(1);
                let cancel_token = CancellationToken::new();
                
                let cancel_token_clone = cancel_token.clone();
                let shared_clone = shared.clone();
                let f_clone = f.clone();

                // Runs the conditional behavior to completion and cancels the other behaviors gracefully when it has completed
                // If any failures are received from other behaviors, this future is terminated as well
                let conditional_clone = conditional.clone();
                let conditional_fut = local_set.spawn_local(async move {
                    select! {
                        result = Self::run(&conditional_clone, shared_clone, f_clone) => {
                            cancel_token_clone.cancel();
                            result
                        }

                        Some(err_result_from_other_behavior) = rx.recv() => {
                            cancel_token_clone.cancel();
                            err_result_from_other_behavior
                        }
                    }
                });

                for behavior in behaviors.clone() {
                    let cancel_token_clone = cancel_token.clone();
                    let tx_clone = tx.clone();
                    let shared_clone = shared.clone();
                    let f_clone = f.clone();
                    // Runs the behavior in a loop till a cancel is received from the conditional behavior
                    // Sends an error condition to the conditional future if any failure
                    local_set.spawn_local(async move {
                        loop {
                            select! {
                                result = Self::run(&behavior, shared_clone.clone(), f_clone.clone()) => {
                                    if result.is_err() {
                                        let _ = tx_clone.send(result).await;
                                        break;
                                    }
                                }

                                _ = cancel_token_clone.cancelled() => { break; }
                            }
                        }
                    });
                }
                local_set.await;
                conditional_fut.await.unwrap()
            }
            // Behavior::WhenAll(_) => todo!(),
            // Behavior::WhenAny(_) => todo!(),
            // Behavior::After(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    enum Calc {
        Inc,
        Dec,
    }

    #[derive(Debug)]
    pub struct Computation {
        num: usize,
    }

    #[tokio::test]
    async fn test_exec() {
        let sequence = Behavior::Sequence(vec![
            Behavior::Wait(Duration::from_millis(1000)),
            Behavior::Action(Calc::Inc),
            Behavior::Action(Calc::Inc),
            Behavior::Action(Calc::Dec),
        ]);

        let data = Rc::new(RefCell::new(Computation { num: 0 }));
        let result = AsyncBehaviorTree::run(
            &sequence,
            data.clone(),
            Rc::new(|action: Calc, shared: Rc<RefCell<Computation>>| async move {
                let mut write = shared.borrow_mut();
                match action {
                    Calc::Inc => write.num += 1,
                    Calc::Dec => write.num -= 1,
                }
                Ok(())
            }),
        )
        .await;
        println!("Data: {:?} {:?}", data, result);
    }
}
