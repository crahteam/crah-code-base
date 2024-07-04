use crate::access::NodeAccess;
use crate::iter::Children;
use crate::{arena::Arena, node::Node};
use std::collections::hash_map::Entry;
use std::fmt::Debug;

pub type Idx = usize;
pub type MovesCount = usize;

/// Represents an arena smart `pointer` to a node.
/// Intelligently stores a stamp on the `MovesCount`
/// of the node it points to at the time it was retrived.
///
/// Can only be retrived by appending a node to the arena.
/// It can't be rawly instantiatied.
///
/// # Example
///
/// ```
/// # use acreage::prelude::*;
/// # use acreage::*;
/// let mut arena = Arena::new();
/// let id: NodeId = arena.append_root(Node::new(0));
/// ```
#[derive(Clone)]
pub struct NodeId {
    pub(crate) idx: Idx,
    pub(crate) at_mc: MovesCount,
}

impl NodeId {
    /// Invalidates the id and returns a new one with an increased moves counter
    pub(crate) fn increase_mc(&self) -> Self {
        Self {
            idx: self.idx,
            at_mc: self.at_mc + 1,
        }
    }

    /// Checks if the id is still pointing to
    /// its original node.
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena = Arena::new();
    /// let p1 = arena.append_root(Node::new("Parent 1"));
    /// let p2 = arena.append_root(Node::new("Parent 2"));
    /// let child = p1.append_child(&p2, &mut arena);
    /// assert!(!p2.validate(&arena));
    /// ```
    pub fn validate<T: Debug>(&self, arena: &Arena<T>) -> bool {
        if arena.moved_idxs.contains_key(&self.idx) {
            // safe unwraps
            return arena.moved_idxs.get(&self.idx).unwrap() == &self.at_mc;
        }
        true
    }

    pub fn as_content<'a, T: Debug>(&'a self, arena: &'a Arena<T>) -> &'a T {
        if self.validate(&arena) {
            return &arena.get(&self).unwrap().as_content();
        } else {
            panic!()
        }
    }

    pub fn children<'a, T: Debug>(&'a self, arena: &'a Arena<T>) -> Children<'a, T> {
        if self.validate(arena) {
            return Children {
                arena,
                idx: self.idx,
                depth: 0,
            };
        }

        todo!()
    }

    pub(crate) fn move_id<T: Debug>(&self, arena: &mut Arena<T>) {
        arena
            .moved_idxs
            .entry(self.idx)
            .and_modify(|mc| *mc += 1)
            .or_insert(1);
        // arguably a good thing? TODO!! to move all the children
        // and invalidating all the ref to em
        // let children = self.children(&arena);

        // for idx in children {
        //    arena
        //        .moved_idxs
        //        .entry(idx)
        //        .and_modify(|mc| *mc += 1)
        //        .or_insert(1);
        // }
    }

    // DESIGN: Should we return a CLONE of the original value or return None instead
    // if it isnt needed ??
    // TODO
    pub(crate) fn renew<T: Debug>(&self, arena: &mut Arena<T>) -> NodeId {
        if self.validate(arena) {
            return match arena.moved_idxs.get(&self.idx) {
                Some(at_mc) => NodeId {
                    at_mc: *at_mc,
                    idx: self.idx,
                },
                None => self.clone(),
            };
        }

        todo!();
    }

    pub(crate) fn move_renew<T: Debug>(&self, arena: &mut Arena<T>) -> NodeId {
        self.move_id(arena);
        self.renew(arena)
    }
}

impl<'a, T: Debug> AppendNode<'a, T> for NodeId {
    ///
    fn append_child(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId {
        let new_idx = arena.next_idx();
        if let Some(c) = arena.get(&self).unwrap().child {
            let mut last_child_idx = c;

            while let Some(i) = arena[c].next {
                last_child_idx = i;
            }

            match node.into() {
                NodeAccess::ById(id) => {
                    if id.validate(&arena) {
                        arena[id.idx].prev = Some(last_child_idx);
                        arena[id.idx].parent = Some(self.idx);
                        arena[last_child_idx].next = Some(id.idx);
                    }
                    return id.clone();
                }
                NodeAccess::Owned(mut n) => {
                    n.prev = Some(last_child_idx);
                    n.parent = Some(self.idx);
                    arena[last_child_idx].next = Some(new_idx);
                    arena.push(n);
                    return NodeId {
                        idx: new_idx,
                        at_mc: 0,
                    };
                }
            }
        } else {
            match node.into() {
                NodeAccess::ById(id) => {
                    if id.validate(&arena) {
                        arena[id.idx].parent = Some(self.idx);
                        arena[self.idx].child = Some(id.idx);
                        return id.increase_mc();
                    } else {
                        todo!()
                    }
                }
                NodeAccess::Owned(mut n) => {
                    n.parent = Some(self.idx);
                    arena[self.idx].child = Some(new_idx);
                    arena.push(n);
                    return NodeId {
                        idx: new_idx,
                        at_mc: 0,
                    };
                }
            }
        }
    }
}

pub trait AppendNode<'a, T: Debug> {
    fn append_child(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId;
}

pub trait InsertNode<'a, T: Debug> {
    fn insert_child(
        &self,
        position: usize,
        node: impl Into<NodeAccess<'a, T>>,
        arena: &mut Arena<T>,
    ) -> NodeId;
    fn insert_prev(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId;
    fn insert_next(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId;
}

pub trait DetachNode<T: Debug> {
    /// drops from the vec and
    fn detach(&self, arena: &mut Arena<T>) -> NodeId;
    fn remove(self, arena: &mut Arena<T>);
}

impl<'a, T: Debug> InsertNode<'a, T> for NodeId {
    /// Insert a child node at a certain position. If the position given is > than the number of children,
    /// the node is appended.
    ///
    /// # Example
    ///
    /// ```
    /// # use acreage::*;
    /// # use acreage::prelude::*;
    ///
    /// let mut arena = Arena::new();
    /// let node = arena.append_root(Node::new(0));
    /// let child = node.insert_child(0, Node::new(1), &mut arena);
    /// assert_eq!(arena.get(&child).unwrap().as_content(), &1);
    /// ```
    fn insert_child(
        &self,
        position: usize,
        node: impl Into<NodeAccess<'a, T>>,
        arena: &mut Arena<T>,
    ) -> NodeId {
        if let Some(child_idx) = arena.get(&self).unwrap().child {
            let mut to_shift = child_idx;

            let mut c = 0;

            while let Some(idx) = arena[to_shift].next {
                to_shift = idx;
                if c == position {
                    break;
                }

                c += 1;
            }

            match node.into() {
                NodeAccess::ById(id) => {
                    if id.validate(&arena) {
                        // fix the sibling

                        let ne = arena[to_shift].next.clone();
                        arena[to_shift].next = Some(id.idx);

                        // fix the new child

                        arena[id.idx].parent = Some(self.idx);
                        arena[id.idx].next = ne;
                        arena[id.idx].prev = Some(to_shift);

                        return id.increase_mc();
                    } else {
                        todo!();
                    }
                }
                NodeAccess::Owned(mut node) => {
                    let new_idx = arena.len();
                    let ne = arena[to_shift].next.clone();
                    arena[to_shift].next = Some(new_idx);

                    node.parent = Some(self.idx);
                    node.next = ne;
                    node.prev = Some(to_shift);

                    arena.push(node);

                    return NodeId {
                        idx: new_idx,
                        at_mc: 0,
                    };
                }
            }
        } else {
            self.append_child(node, arena)
        }
    }

    ///
    fn insert_prev(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId {
        let pr = arena[self.idx].prev.clone();

        match node.into() {
            NodeAccess::ById(id) => {
                if arena.get(&id).is_some() {
                    arena[id.idx].prev = pr;
                    arena[id.idx].next = Some(self.idx);
                    arena[id.idx].parent = arena[self.idx].parent.clone();
                    return id.increase_mc();
                } else {
                    todo!()
                }
            }

            NodeAccess::Owned(mut n) => {
                let new_idx = arena.len();

                n.prev = pr;
                n.next = Some(self.idx);
                n.parent = arena[self.idx].parent.clone();

                arena.push(n);
                return NodeId {
                    idx: new_idx,
                    at_mc: 0,
                };
            }
        }
    }

    fn insert_next(&self, node: impl Into<NodeAccess<'a, T>>, arena: &mut Arena<T>) -> NodeId {
        let nx = arena[self.idx].next.clone();

        match node.into() {
            NodeAccess::ById(id) => {
                if arena.get(&id).is_some() {
                    arena[id.idx].next = nx;
                    arena[id.idx].prev = Some(self.idx);
                    arena[id.idx].parent = arena[self.idx].parent.clone();

                    return id.increase_mc();
                } else {
                    todo!()
                }
            }

            NodeAccess::Owned(mut n) => {
                let new_idx = arena.len();

                n.next = nx;
                n.prev = Some(self.idx);
                n.parent = arena[self.idx].parent.clone();

                arena.push(n);
                return NodeId {
                    idx: new_idx,
                    at_mc: 0,
                };
            }
        }
    }
}
