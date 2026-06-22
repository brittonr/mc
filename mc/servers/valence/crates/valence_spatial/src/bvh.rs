use std::iter::FusedIterator;

use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::Bounded3D;

mod rebuild;
mod traverse;

/// Public API compatibility: `Bvh` is the crate's exposed bounding-volume
/// hierarchy type.
#[derive(Clone, Debug)]
#[allow(unknown_lints, path_segment_repetition)]
pub struct Bvh<T> {
    internal_nodes: Vec<InternalNode>,
    leaf_nodes: Vec<T>,
    root: NodeIdx,
}

#[derive(Clone, Debug)]
struct InternalNode {
    bb: vek::Aabb<f64>,
    left: NodeIdx,
    right: NodeIdx,
}

type NodeIdx = usize;

const EMPTY_NODE: NodeIdx = NodeIdx::MAX;
const BINARY_TREE_NODE_FACTOR: usize = 2;

fn empty_bounds() -> vek::Aabb<f64> {
    let origin = vek::Vec3::new(0.0, 0.0, 0.0);
    vek::Aabb {
        min: origin,
        max: origin,
    }
}

impl<T: Bounded3D + Send + Sync> Bvh<T> {
    pub fn new() -> Self {
        Self {
            internal_nodes: vec![],
            leaf_nodes: vec![],
            root: EMPTY_NODE,
        }
    }

    pub fn rebuild<I: IntoIterator<Item = T>>(&mut self, leaves: I) {
        self.internal_nodes.clear();
        self.leaf_nodes.clear();

        self.leaf_nodes.extend(leaves);

        let leaf_count = self.leaf_nodes.len();

        if leaf_count == 0 {
            return;
        }

        let Some(internal_count) = leaf_count.checked_sub(1) else {
            return;
        };
        self.internal_nodes.reserve_exact(internal_count);
        self.internal_nodes.resize(
            internal_count,
            InternalNode {
                bb: empty_bounds(),
                left: EMPTY_NODE,
                right: EMPTY_NODE,
            },
        );

        let Some(leaf_count_idx) = valid_leaf_count(leaf_count) else {
            self.internal_nodes.clear();
            self.leaf_nodes.clear();
            self.root = EMPTY_NODE;
            return;
        };

        let id = self.leaf_nodes[0].aabb();
        let scene_bounds = self
            .leaf_nodes
            .par_iter()
            .map(|l| l.aabb())
            .reduce(|| id, vek::Aabb::union);

        self.root = rebuild::tree(
            0,
            scene_bounds,
            &mut self.internal_nodes,
            &mut self.leaf_nodes,
            leaf_count_idx,
        )
        .0;

        debug_assert!(
            self.leaf_nodes
                .len()
                .checked_sub(1)
                .is_some_and(|internal_count| internal_count == self.internal_nodes.len()),
            "internal node count matches leaf count"
        );
    }

    pub fn traverse(&self) -> Option<Node<'_, T>> {
        if !self.leaf_nodes.is_empty() {
            Some(Node::from_idx(self, self.root))
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> + FusedIterator + Clone + '_ {
        self.leaf_nodes.iter()
    }

    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> + FusedIterator + '_ {
        self.leaf_nodes.iter_mut()
    }

    pub fn par_iter(&self) -> impl IndexedParallelIterator<Item = &T> + Clone + '_ {
        self.leaf_nodes.par_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl IndexedParallelIterator<Item = &mut T> + '_ {
        self.leaf_nodes.par_iter_mut()
    }
}

impl<T: Bounded3D + Send + Sync> Default for Bvh<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn valid_leaf_count(leaf_count: usize) -> Option<NodeIdx> {
    let max_leaf_idx = leaf_count.checked_sub(1)?;
    leaf_count.checked_add(max_leaf_idx)?;
    Some(leaf_count)
}

fn max_tree_node_count(leaf_count: usize) -> usize {
    match leaf_count
        .checked_mul(BINARY_TREE_NODE_FACTOR)
        .and_then(|node_count| node_count.checked_sub(1))
    {
        Some(node_count) => node_count,
        None => leaf_count,
    }
}

#[derive(Debug)]
pub enum Node<'a, T> {
    Internal(Internal<'a, T>),
    Leaf(&'a T),
}

impl<'a, T> Node<'a, T> {
    fn from_idx(bvh: &'a Bvh<T>, idx: NodeIdx) -> Self {
        let internal_count = bvh.internal_nodes.len();
        if idx < internal_count {
            return Self::Internal(Internal { bvh, idx });
        }

        let leaf_idx = idx.saturating_sub(internal_count);
        Self::Leaf(&bvh.leaf_nodes[leaf_idx])
    }
}

impl<T: Bounded3D> Bounded3D for Node<'_, T> {
    fn aabb(&self) -> vek::Aabb<f64> {
        match self {
            Node::Internal(int) => int.aabb(),
            Node::Leaf(t) => t.aabb(),
        }
    }
}

impl<T> Clone for Node<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Node<'_, T> {}

#[derive(Debug)]
pub struct Internal<'a, T> {
    bvh: &'a Bvh<T>,
    idx: NodeIdx,
}

impl<'a, T> Internal<'a, T> {
    pub fn split(self) -> (vek::Aabb<f64>, Node<'a, T>, Node<'a, T>) {
        let internal = &self.bvh.internal_nodes[self.idx];

        let bb = internal.bb;
        let left = Node::from_idx(self.bvh, internal.left);
        let right = Node::from_idx(self.bvh, internal.right);

        (bb, left, right)
    }
}

impl<T> Bounded3D for Internal<'_, T> {
    fn aabb(&self) -> vek::Aabb<f64> {
        self.bvh.internal_nodes[self.idx].bb
    }
}

impl<T> Clone for Internal<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Internal<'_, T> {}
