//! `test_deps` allows developers to define dependencies among tests.

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, PoisonError};
pub use test_deps_if::deps;

lazy_static! {
    static ref TEST_DEPS_REG: Mutex<(HashMap<String, ()>, HashMap<String, Vec<Arc<Condvar>>>)> = {
        let completed = HashMap::new();
        let waitlist = HashMap::new();
        Mutex::new((completed, waitlist))
    };
}

#[doc(hidden)]
#[derive(Debug)]
pub enum TestDepsHelperError {
    ThreadSync(String),
    CorruptedDeps(String),
    Generic(String),
}

impl<T> From<PoisonError<T>> for TestDepsHelperError {
    fn from(_err: PoisonError<T>) -> TestDepsHelperError {
        TestDepsHelperError::ThreadSync(String::from("A mutex is poisoned"))
    }
}

#[doc(hidden)]
pub fn target_completed(target: &String) -> Result<(), TestDepsHelperError> {
    let mut reg = TEST_DEPS_REG.lock()?;
    if reg.0.insert(target.clone(), ()).is_some() {
        Err(TestDepsHelperError::CorruptedDeps(format!(
            "A target duplicated on the completed list: {}",
            target
        )))
    } else {
        if let Some(cvs) = reg.1.get(target) {
            for cv in cvs {
                cv.notify_one();
            }
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn ensure_prereqs(prereqs: &Vec<String>) -> Result<(), TestDepsHelperError> {
    let mut reg = TEST_DEPS_REG.lock()?;
    if !check_readiness(&reg.0, prereqs) {
        let cv = Arc::new(Condvar::new());
        for prereq in prereqs {
            if let Some(cvs) = reg.1.get_mut(prereq) {
                cvs.push(cv.clone());
            } else {
                reg.1.insert(prereq.clone(), vec![cv.clone()]);
            }
        }
        while !check_readiness(&reg.0, prereqs) {
            reg = cv.wait(reg)?;
        }
    }
    Ok(())
}

fn check_readiness(completed: &HashMap<String, ()>, prereqs: &Vec<String>) -> bool {
    for prereq in prereqs {
        if !completed.contains_key(prereq) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn target_completed_no_one_is_waiting_for() {
        target_completed(&String::from("NO_ONE_IS_WAITING")).unwrap();
    }

    #[test]
    fn no_prereq_passes_immediately() {
        ensure_prereqs(&vec![]).unwrap();
    }

    #[test]
    fn wait_one_prereq() {
        ensure_prereqs(&vec![String::from("FIRST_TARGET")]).unwrap();
    }

    #[test]
    fn wait_two_prereqs() {
        ensure_prereqs(&vec![
            String::from("FIRST_TARGET"),
            String::from("SECOND_TARGET"),
        ])
        .unwrap();
    }

    #[test]
    fn first_target_completed() {
        // ensure wait_one_prereq() and wait_two_prereqs() go first
        thread::sleep(time::Duration::from_secs_f64(0.75));
        target_completed(&String::from("FIRST_TARGET")).unwrap();
    }

    #[test]
    fn second_target_completed() {
        // ensure wait_one_prereq() and wait_two_prereqs() go first
        thread::sleep(time::Duration::from_secs_f64(0.75));
        target_completed(&String::from("SECOND_TARGET")).unwrap();
    }

    #[test]
    fn prereqs_already_satisfied() {
        // ensure target_completed_before_waiter_comes() goes first
        thread::sleep(time::Duration::from_secs_f64(0.75));
        ensure_prereqs(&vec![String::from("TARGET_GOING_FIRST")]).unwrap();
    }

    #[test]
    fn target_completed_before_waiter_comes() {
        target_completed(&String::from("TARGET_GOING_FIRST")).unwrap();
    }

    #[test]
    #[should_panic(expected = "A target duplicated on the completed list: COMPLETES_TWICE")]
    fn target_completed_twice_unexpectedly() {
        if let Err(_) = target_completed(&String::from("COMPLETES_TWICE")) {
            panic!("should not panic here");
        }
        target_completed(&String::from("COMPLETES_TWICE")).unwrap();
    }
}
