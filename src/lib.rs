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
