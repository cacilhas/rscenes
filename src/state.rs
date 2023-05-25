use std::{cell::RefCell, rc::Rc};

use crate::Scene;

pub enum State {
    Keep,
    New(Rc<RefCell<dyn Scene>>),
    Previous(usize),
}
