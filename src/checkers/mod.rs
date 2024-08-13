use std::fmt::Debug;

use crate::runtime::DepCheckerContext;

use self::js::JSFamily;

mod js;

pub fn get_js_checkers() -> Vec<CheckerKind> {
    vec![CheckerKind::JSFamily(JSFamily)]
}

#[derive(Debug)]
pub enum CheckerKind {
    JSFamily(JSFamily),
}

impl Default for CheckerKind {
    fn default() -> Self {
        CheckerKind::JSFamily(JSFamily)
    }
}

impl Checker for CheckerKind {
    fn run(&self, ctx: &DepCheckerContext) {
        match self {
            CheckerKind::JSFamily(checker) => checker.run(ctx),
        }
    }
}

pub trait Checker: Sized + Default + Debug {
    /// Visit each file to detect dependencies
    fn run(&self, _ctx: &DepCheckerContext) {}
}
