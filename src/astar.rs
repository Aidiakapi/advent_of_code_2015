
#![allow(dead_code)]

use num::traits::Zero;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Add;

pub trait Node = Clone + Eq + Hash;
pub trait Cost = Clone + Ord + Add + Zero;

#[derive(Debug, Clone)]
pub struct AStar<N: Node, C: Cost> {
    meta: HashMap<N, Meta<N, C>>,
    open: BinaryHeap<Open<N, C>>,
    path: Vec<(N, C)>,
}

#[derive(Debug, Clone)]
struct Meta<N: Node, C: Cost> {
    is_closed: bool,
    heuristic: C,
    path: C,
    parent: Option<N>,
}

#[derive(Debug, Clone, Eq)]
struct Open<N: Node, C: Cost> {
    cost: C,
    node: N,
    counter: usize,
}

impl<N: Node, C: Cost> PartialEq for Open<N, C> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N: Node, C: Cost> PartialOrd for Open<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<N: Node, C: Cost> Ord for Open<N, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(self.counter.cmp(&other.counter))
    }
}

impl<N: Node, C: Cost> AStar<N, C> {
    pub fn new() -> Self {
        AStar {
            meta: HashMap::new(),
            open: BinaryHeap::new(),
            path: Vec::new(),
        }
    }

    pub fn into_last_path(self) -> Vec<(N, C)> {
        self.path
    }

    pub fn solve<FN, FH, FD, NI>(
        &mut self,
        init: N,
        mut next: FN,
        mut heuristic: FH,
        mut is_done: FD,
    ) -> Option<&Vec<(N, C)>>
    where
        FN: FnMut(&N) -> NI,
        FH: FnMut(&N) -> C,
        FD: FnMut(&N) -> bool,
        NI: IntoIterator<Item = (N, C)>,
    {
        // Used to get FIFO behaviour from the open set
        let mut counter = 0;
        self.path.clear();
        let init_heuristic = heuristic(&init);
        let init_meta = Meta {
            is_closed: false,
            path: C::zero(),
            heuristic: init_heuristic.clone(),
            parent: None,
        };
        self.meta.insert(init.clone(), init_meta);
        let init_open = Open {
            node: init,
            cost: init_heuristic,
            counter,
        };
        self.open.push(init_open);

        while let Some(open) = self.open.pop() {
            let meta = self.meta.get_mut(&open.node).unwrap();
            // This can happen if the same node was inserted multiple times into the
            // open set, because a later found route to the same node actually had a
            // shorter total length.
            if meta.is_closed {
                continue;
            }
            meta.is_closed = true;

            if is_done(&open.node) {
                // Reconstruct the path
                let mut current_node = Some(&open.node);
                while let Some(n) = current_node {
                    let meta = &self.meta[&n];
                    self.path.push((n.clone(), meta.path.clone()));
                    current_node = meta.parent.as_ref();
                }

                self.path.reverse();

                self.open.clear();
                self.meta.clear();
                return Some(&self.path);
            }
            let path_cost = meta.path.clone();
            for (node, edge_cost) in next(&open.node) {
                let cost = match self.meta.get_mut(&node) {
                    Some(meta) => {
                        // If the node was already seen, and is in closed,
                        // the shortest route is already established, and
                        // there is no need to revisit the node.
                        if meta.is_closed {
                            continue;
                        }
                        // If the other node is already in the open set
                        // but the cost through this parent node is cheaper
                        // it has to be updated.
                        let path_cost = edge_cost + path_cost.clone();
                        if meta.path <= path_cost {
                            continue;
                        }
                        // Update price
                        meta.path = path_cost.clone();
                        meta.parent = Some(open.node.clone());
                        path_cost
                    }
                    // New node
                    None => {
                        let path_cost = edge_cost + path_cost.clone();
                        let heuristic_cost = heuristic(&node);
                        self.meta.insert(
                            node.clone(),
                            Meta {
                                is_closed: false,
                                path: path_cost.clone(),
                                heuristic: heuristic_cost.clone(),
                                parent: Some(open.node.clone()),
                            },
                        );
                        path_cost + heuristic_cost
                    }
                };
                counter += 1;
                self.open.push(Open {
                    node: node,
                    cost,
                    counter,
                });
            }
        }

        self.open.clear();
        self.meta.clear();
        None
    }
}
