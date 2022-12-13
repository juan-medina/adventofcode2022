use crate::node::Node;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct NavigationStep {
    pub from: Node,
    pub num_step: usize,
}

impl NavigationStep {
    pub fn new(from: Node, step: usize) -> Self {
        Self {
            from,
            num_step: step,
        }
    }
}
