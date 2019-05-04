#[macro_use]
mod term;

#[macro_use]
mod path;

mod banner;
mod cargo;
mod dependencies;
mod env;
mod error;
mod manifest;
mod message;
mod normalize;
mod run;

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::thread;

pub struct TestCases {
    runner: RefCell<Runner>,
}

struct Runner {
    tests: Vec<Test>,
}

#[derive(Clone)]
struct Test {
    path: PathBuf,
    expected: Expected,
}

#[derive(Copy, Clone)]
enum Expected {
    Pass,
    CompileFail,
}

impl TestCases {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        TestCases {
            runner: RefCell::new(Runner { tests: Vec::new() }),
        }
    }

    pub fn pass<P: AsRef<Path>>(&self, path: P) {
        self.runner.borrow_mut().tests.push(Test {
            path: path.as_ref().to_owned(),
            expected: Expected::Pass,
        });
    }

    pub fn compile_fail<P: AsRef<Path>>(&self, path: P) {
        self.runner.borrow_mut().tests.push(Test {
            path: path.as_ref().to_owned(),
            expected: Expected::CompileFail,
        });
    }
}

impl Drop for TestCases {
    fn drop(&mut self) {
        if !thread::panicking() {
            self.runner.borrow_mut().run();
        }
    }
}
