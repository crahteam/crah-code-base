use crate::{id::Idx, Arena, Node, NodeId};
use std::fmt::Debug;

// should be verifiedd the existence and validation of the nodeid before being constructed
pub struct PrevSiblings<'a, T: Debug> {
    arena: &'a Arena<T>,
    idx: Idx,
}

impl<'a, T: Debug> Iterator for PrevSiblings<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx = if let Some(i) = self.arena[self.idx].prev {
            i
        } else {
            return None;
        };
        Some(&self.arena[self.idx])
    }
}

pub struct NextSiblings<'a, T: Debug> {
    arena: &'a Arena<T>,
    idx: Idx,
}

impl<'a, T: Debug> Iterator for NextSiblings<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx = if let Some(i) = self.arena[self.idx].next {
            i
        } else {
            return None;
        };
        Some(&self.arena[self.idx])
    }
}

pub struct Children<'a, T: Debug> {
    pub(crate) arena: &'a Arena<T>,
    pub(crate) idx: Idx,
    pub(crate) depth: usize,
}

impl<'a, T: Debug> Iterator for Children<'a, T> {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.arena[self.idx].child {
            self.idx = c;
            self.depth += 1;
            //return Some(self.arena[c]);
            return Some(c);
        }

        if let Some(s) = self.arena[self.idx].next {
            self.idx = s;
            //        return Some(self.arena[s]);
            return Some(s);
        }

        let mut parent = self.arena[self.idx].parent;
        if self.depth > 0 {
            if let Some(p) = self.arena[self.idx].parent {
                while let None = self.arena[p].next {
                    parent = self.arena[p].parent;
                    self.depth -= 1;

                    if self.depth == 0 {
                        return None;
                    }
                }

                if let Some(n) = self.arena[p].next {
                    self.idx = n;
                    //return Some(self.arena[n]);
                    return Some(n);
                }

                unreachable!();
            }

            unreachable!();
        }

        None
    }
}
