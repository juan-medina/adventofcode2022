use crate::step::NavigationStep;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Node {
    pub x: usize,
    pub y: usize,
}

impl Node {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn step(&self, step_num: usize) -> NavigationStep {
        NavigationStep::new(self.clone(), step_num)
    }
    pub fn get_near_nodes(&self, width: usize, height: usize) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();

        if self.x > 0 {
            nodes.push(Node::new(self.x - 1, self.y));
        }
        if self.x < width - 1 {
            nodes.push(Node::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            nodes.push(Node::new(self.x, self.y - 1));
        }
        if self.y < height - 1 {
            nodes.push(Node::new(self.x, self.y + 1));
        }

        nodes
    }
}
