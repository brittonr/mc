use crate::Bounded3D;

type Bounds = vek::Aabb<f64>;

const MIDPOINT_DIVISOR: f64 = 2.0;
const AABB_COLLAPSE_EPSILON_SCALE: f64 = 100.0;

struct SplitPlane {
    index: usize,
    left_bounds: Bounds,
    right_bounds: Bounds,
}

enum SplitDecision {
    Ready(SplitPlane),
    Retry(Bounds),
}

#[derive(Clone, Copy)]
struct LeafResultCtx {
    idx: super::NodeIdx,
    total_leaf_count: super::NodeIdx,
}

#[derive(Clone, Copy)]
struct NodeBase {
    idx: super::NodeIdx,
}

#[derive(Clone, Copy)]
struct AxisRange {
    lower_bound: f64,
    upper_bound: f64,
}

pub(super) fn tree<T: Send + Bounded3D>(
    idx: super::NodeIdx,
    mut bounds: Bounds,
    internal_nodes: &mut [super::InternalNode],
    leaf_nodes: &mut [T],
    total_leaf_count: super::NodeIdx,
) -> (super::NodeIdx, Bounds) {
    debug_assert!(
        leaf_nodes
            .len()
            .checked_sub(1)
            .is_some_and(|internal_count| internal_count == internal_nodes.len()),
        "internal node count matches leaf count"
    );

    if leaf_nodes.len() == 1 {
        return leaf_result(
            LeafResultCtx {
                idx,
                total_leaf_count,
            },
            leaf_nodes,
        );
    }

    for _ in 0..super::max_tree_node_count(leaf_nodes.len()) {
        debug_assert!(bounds.is_valid());
        match normalize_split(choose_split(bounds, leaf_nodes), leaf_nodes.len()) {
            SplitDecision::Ready(split) => {
                return assemble_split(idx, split, internal_nodes, leaf_nodes, total_leaf_count);
            }
            SplitDecision::Retry(next_bounds) => bounds = next_bounds,
        }
    }

    (idx, bounds)
}

fn leaf_result<T: Bounded3D>(ctx: LeafResultCtx, leaf_nodes: &[T]) -> (super::NodeIdx, Bounds) {
    let Some(leaf_base_idx) = ctx.total_leaf_count.checked_sub(1) else {
        return (ctx.idx, leaf_nodes[0].aabb());
    };
    let Some(node_idx) = leaf_base_idx.checked_add(ctx.idx) else {
        return (ctx.idx, leaf_nodes[0].aabb());
    };
    (node_idx, leaf_nodes[0].aabb())
}

fn choose_split<T: Bounded3D>(bounds: Bounds, leaf_nodes: &mut [T]) -> SplitPlane {
    let dims = bounds.max - bounds.min;

    if dims.x >= dims.y && dims.x >= dims.z {
        return split_x(bounds, leaf_nodes);
    }
    if dims.y >= dims.x && dims.y >= dims.z {
        return split_y(bounds, leaf_nodes);
    }
    split_z(bounds, leaf_nodes)
}

fn split_x<T: Bounded3D>(bounds: Bounds, leaf_nodes: &mut [T]) -> SplitPlane {
    let mid = middle(AxisRange::new(bounds.min.x, bounds.max.x));
    let [left_bounds, right_bounds] = bounds.split_at_x(mid);
    let index = partition(leaf_nodes, |leaf| {
        middle(AxisRange::new(leaf.aabb().min.x, leaf.aabb().max.x)) <= mid
    });
    SplitPlane {
        index,
        left_bounds,
        right_bounds,
    }
}

fn split_y<T: Bounded3D>(bounds: Bounds, leaf_nodes: &mut [T]) -> SplitPlane {
    let mid = middle(AxisRange::new(bounds.min.y, bounds.max.y));
    let [left_bounds, right_bounds] = bounds.split_at_y(mid);
    let index = partition(leaf_nodes, |leaf| {
        middle(AxisRange::new(leaf.aabb().min.y, leaf.aabb().max.y)) <= mid
    });
    SplitPlane {
        index,
        left_bounds,
        right_bounds,
    }
}

fn split_z<T: Bounded3D>(bounds: Bounds, leaf_nodes: &mut [T]) -> SplitPlane {
    let mid = middle(AxisRange::new(bounds.min.z, bounds.max.z));
    let [left_bounds, right_bounds] = bounds.split_at_z(mid);
    let index = partition(leaf_nodes, |leaf| {
        middle(AxisRange::new(leaf.aabb().min.z, leaf.aabb().max.z)) <= mid
    });
    SplitPlane {
        index,
        left_bounds,
        right_bounds,
    }
}

fn normalize_split(mut split: SplitPlane, leaf_count: usize) -> SplitDecision {
    debug_assert!(
        leaf_count > 1,
        "split normalization requires multiple leaves"
    );
    if split.index == 0 {
        if bounds_collapsed(split.right_bounds) {
            split.index = split.index.saturating_add(1);
            return SplitDecision::Ready(split);
        }
        return SplitDecision::Retry(split.right_bounds);
    }

    if split.index == leaf_count {
        if bounds_collapsed(split.left_bounds) {
            let Some(index) = split.index.checked_sub(1) else {
                return SplitDecision::Retry(split.left_bounds);
            };
            split.index = index;
            return SplitDecision::Ready(split);
        }
        return SplitDecision::Retry(split.left_bounds);
    }

    SplitDecision::Ready(split)
}

fn bounds_collapsed(bounds: Bounds) -> bool {
    approx::abs_diff_eq!(
        bounds.min,
        bounds.max,
        epsilon = f64::EPSILON * AABB_COLLAPSE_EPSILON_SCALE
    )
}

fn assemble_split<T: Send + Bounded3D>(
    idx: super::NodeIdx,
    split: SplitPlane,
    internal_nodes: &mut [super::InternalNode],
    leaf_nodes: &mut [T],
    total_leaf_count: super::NodeIdx,
) -> (super::NodeIdx, Bounds) {
    debug_assert!(split.index > 0, "split index leaves a non-empty left side");
    let base = NodeBase { idx };
    let Some(right_idx) = base.offset(split.index) else {
        return (idx, split.left_bounds.union(split.right_bounds));
    };
    let Some(current_idx) = base.parent_for_split(split.index) else {
        return (idx, split.left_bounds.union(split.right_bounds));
    };
    let Some(parent_offset) = split.index.checked_sub(1) else {
        return (idx, split.left_bounds.union(split.right_bounds));
    };

    let (leaves_left, leaves_right) = leaf_nodes.split_at_mut(split.index);
    let (internal_left, internal_after_left) = internal_nodes.split_at_mut(parent_offset);
    let Some((internal, internal_right)) = internal_after_left.split_first_mut() else {
        return (idx, split.left_bounds.union(split.right_bounds));
    };

    let ((left, left_bounds), (right, right_bounds)) = rayon::join(
        || {
            tree(
                idx,
                split.left_bounds,
                internal_left,
                leaves_left,
                total_leaf_count,
            )
        },
        || {
            tree(
                right_idx,
                split.right_bounds,
                internal_right,
                leaves_right,
                total_leaf_count,
            )
        },
    );

    internal.bb = left_bounds.union(right_bounds);
    internal.left = left;
    internal.right = right;

    (current_idx, internal.bb)
}

fn partition<T>(values: &mut [T], mut pred: impl FnMut(&T) -> bool) -> usize {
    debug_assert!(!values.is_empty(), "partition expects a non-empty slice");
    let mut it = values.iter_mut();
    let mut true_count = 0;

    while let Some(head) = it.find(|x| {
        if pred(x) {
            true_count += 1;
            false
        } else {
            true
        }
    }) {
        if let Some(tail) = it.rfind(|x| pred(x)) {
            std::mem::swap(head, tail);
            true_count += 1;
        } else {
            break;
        }
    }
    true_count
}

impl NodeBase {
    fn offset(self, offset: usize) -> Option<super::NodeIdx> {
        self.idx.checked_add(offset)
    }

    fn parent_for_split(self, split: usize) -> Option<super::NodeIdx> {
        self.offset(split)?.checked_sub(1)
    }
}

impl AxisRange {
    fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }
}

fn middle(range: AxisRange) -> f64 {
    range.lower_bound + (range.upper_bound - range.lower_bound) / MIDPOINT_DIVISOR
}
