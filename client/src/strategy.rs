use crate::{
    analyzer::Analyzer,
    strategy::{behavior::Behavior, condition::Condition},
};

pub mod behavior;
pub mod condition;
pub mod target;

pub struct Strategy {
    tree: StrategyNode,
}

impl Strategy {
    pub fn new(branches: Vec<(Box<Condition>, Box<StrategyNode>)>) -> Self {
        Self {
            tree: StrategyNode::Branch(branches),
        }
    }

    pub fn next_behavior(&mut self, analyzer: &Analyzer) -> Option<(Priority, Box<Behavior>)> {
        self.tree.next_behavior(analyzer)
    }
}

pub enum StrategyNode {
    Branch(Vec<(Box<Condition>, Box<StrategyNode>)>),
    Leaf((Priority, Box<Behavior>)),
}

impl StrategyNode {
    pub fn next_behavior(&mut self, analyzer: &Analyzer) -> Option<(Priority, Box<Behavior>)> {
        match self {
            StrategyNode::Branch(nodes) => {
                for (condition, node) in nodes.iter_mut() {
                    if condition.evaluate(analyzer) {
                        return node.next_behavior(analyzer);
                    }
                }
                None
            }
            StrategyNode::Leaf(leaf) => Some(leaf.clone()),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
}
