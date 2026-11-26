//! Research fixture: launch dispatch is not foreground acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dispatch { Accepted, Rejected }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome { Confirmed, Rejected, NotObserved, Uncertain }
#[derive(Clone, Copy)]
struct Observation<'a> {
    request_id: &'a str,
    package: &'a str,
    observed_at: u64,
    visible: bool,
}
fn classify(id: &str, target: &str, sent_at: u64, deadline: u64,
            dispatch: Dispatch, observation: Option<Observation<'_>>) -> Outcome {
    if dispatch == Dispatch::Rejected { return Outcome::Rejected; }
    let Some(o) = observation else { return Outcome::Uncertain; };
    if o.request_id != id || o.package != target ||
       o.observed_at < sent_at || o.observed_at > deadline {
        return Outcome::Uncertain;
    }
    if o.visible { Outcome::Confirmed } else { Outcome::NotObserved }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn obs<'a>(id: &'a str, package: &'a str, time: u64, visible: bool) -> Observation<'a> {
        Observation { request_id: id, package, observed_at: time, visible }
    }
    fn run(dispatch: Dispatch, o: Option<Observation<'_>>) -> Outcome {
        classify("r1", "com.example.fixture", 100, 200, dispatch, o)
    }
    #[test] fn zero_exit_is_not_acceptance() { assert_eq!(run(Dispatch::Accepted,None),Outcome::Uncertain); }
    #[test] fn denied_dispatch_is_rejected() { assert_eq!(run(Dispatch::Rejected,None),Outcome::Rejected); }
    #[test] fn visible_target_confirms() { assert_eq!(run(Dispatch::Accepted,Some(obs("r1","com.example.fixture",120,true))),Outcome::Confirmed); }
    #[test] fn negative_observation() { assert_eq!(run(Dispatch::Accepted,Some(obs("r1","com.example.fixture",120,false))),Outcome::NotObserved); }
    #[test] fn wrong_app() { assert_eq!(run(Dispatch::Accepted,Some(obs("r1","com.other",120,true))),Outcome::Uncertain); }
    #[test] fn wrong_request() { assert_eq!(run(Dispatch::Accepted,Some(obs("r2","com.example.fixture",120,true))),Outcome::Uncertain); }
    #[test] fn stale_observation() { assert_eq!(run(Dispatch::Accepted,Some(obs("r1","com.example.fixture",99,true))),Outcome::Uncertain); }
    #[test] fn expired_observation() { assert_eq!(run(Dispatch::Accepted,Some(obs("r1","com.example.fixture",201,true))),Outcome::Uncertain); }
}
