use crate::{Bounded3D, SpatialIndex};

#[derive(Clone, Copy)]
struct RayVisit<'a, O> {
    node: super::Node<'a, O>,
    near: f64,
    far: f64,
}

fn ray_visit<'a, O: Bounded3D>(
    node: super::Node<'a, O>,
    origin: vek::Vec3<f64>,
    direction: vek::Vec3<f64>,
) -> Option<RayVisit<'a, O>> {
    let (near, far) = crate::ray_box_intersect(origin, direction, node.aabb())?;
    Some(RayVisit { node, near, far })
}

fn visits_for_stack<'a, O>(left: RayVisit<'a, O>, right: RayVisit<'a, O>) -> [RayVisit<'a, O>; 2] {
    if left.near < right.near {
        return [right, left];
    }
    [left, right]
}

impl<O: Bounded3D + Send + Sync> SpatialIndex for super::Bvh<O> {
    type Object = O;

    fn query<C, F, T>(&self, mut collides: C, mut f: F) -> Option<T>
    where
        C: FnMut(vek::Aabb<f64>) -> bool,
        F: FnMut(&O) -> Option<T>,
    {
        let max_visits = super::max_tree_node_count(self.leaf_nodes.len());
        debug_assert!(
            max_visits >= self.leaf_nodes.len(),
            "tree visit bound covers every leaf"
        );
        let mut stack = Vec::with_capacity(max_visits);
        stack.push(self.traverse()?);

        for _ in 0..max_visits {
            let Some(node) = stack.pop() else {
                return None;
            };

            match node {
                super::Node::Internal(int) => {
                    let (bb, left, right) = int.split();
                    if collides(bb) {
                        stack.push(right);
                        stack.push(left);
                    }
                }
                super::Node::Leaf(leaf) => {
                    if collides(leaf.aabb()) {
                        if let Some(value) = f(leaf) {
                            return Some(value);
                        }
                    }
                }
            }
        }

        None
    }

    fn raycast<F>(
        &self,
        origin: vek::Vec3<f64>,
        direction: vek::Vec3<f64>,
        mut f: F,
    ) -> Option<crate::RaycastHit<'_, O>>
    where
        F: FnMut(crate::RaycastHit<O>) -> bool,
    {
        debug_assert!(
            direction.is_normalized(),
            "the ray direction must be normalized"
        );

        let root = ray_visit(self.traverse()?, origin, direction)?;
        let max_visits = super::max_tree_node_count(self.leaf_nodes.len());
        let mut stack = Vec::with_capacity(max_visits);
        let mut hit = None;
        stack.push(root);

        for _ in 0..max_visits {
            let Some(visit) = stack.pop() else {
                return hit;
            };
            if hit.as_ref().is_some_and(|hit| hit.near <= visit.near) {
                continue;
            }

            match visit.node {
                super::Node::Internal(int) => {
                    let (_, left, right) = int.split();
                    match (
                        ray_visit(left, origin, direction),
                        ray_visit(right, origin, direction),
                    ) {
                        (Some(left), Some(right)) => {
                            for visit in visits_for_stack(left, right) {
                                stack.push(visit);
                            }
                        }
                        (Some(left), None) => stack.push(left),
                        (None, Some(right)) => stack.push(right),
                        (None, None) => {}
                    }
                }
                super::Node::Leaf(leaf) => {
                    let this_hit = crate::RaycastHit {
                        object: leaf,
                        near: visit.near,
                        far: visit.far,
                    };
                    if f(this_hit) {
                        hit = Some(this_hit);
                    }
                }
            }
        }

        hit
    }
}
