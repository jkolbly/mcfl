use std::{collections::HashMap, fmt::Debug};

use crate::{
    error::TreeError,
    id_tracker::{self, ID_TRACKER},
};

/// Generic tree struct that stores a vector of nodes which can be accessed with their ID's.
pub struct Tree<T> {
    nodes: HashMap<NodeId, Node<T>>,
    next_index: usize,
    root: Option<NodeId>,
    id: usize,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Tree<T> {
    /// Create a new empty tree with no root
    pub fn new() -> Tree<T> {
        Tree {
            nodes: HashMap::new(),
            next_index: 0,
            root: None,
            id: ID_TRACKER.lock().unwrap().get_id(),
        }
    }

    /// Create a new node and return its ID. If this tree has no root, make the new node the root
    pub fn new_node(&mut self, data: T) -> NodeId {
        let new_id = NodeId {
            id: self.next_index,
            tree_id: self.id,
        };
        self.next_index += 1;

        self.nodes.insert(
            new_id,
            Node {
                parent: None,
                previous_sibling: None,
                next_sibling: None,
                children: Vec::new(),
                depth: 0,
                data,
            },
        );

        if self.root.is_none() {
            self.root = Some(new_id);
        }

        new_id
    }

    /// Get the data for the node specified by `id` or error if node doesn't exist
    pub fn get_node(&self, id: NodeId) -> Result<&T, TreeError> {
        if id.tree_id != self.id {
            return Err(TreeError::MismatchedTreeAndNodeID {
                node_id: id,
                tree_id: self.id,
            });
        }
        if let Some(node) = self.nodes.get(&id) {
            Ok(&node.data)
        } else {
            Err(TreeError::NodeNotFound { node_id: id })
        }
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Result<&mut T, TreeError> {
        if let Some(node) = self.nodes.get_mut(&id) {
            Ok(&mut node.data)
        } else {
            Err(TreeError::NodeNotFound { node_id: id })
        }
    }

    /// Append `child` to `parent` so that `child` is now one of `parent`'s children
    pub fn append_to(&mut self, parent: NodeId, child: NodeId) -> Result<(), TreeError> {
        let parent_depth: u32;
        let sibling_id = {
            let p_node = self.get_treenode_mut(parent)?;
            let sibling_id = p_node.children.last().cloned();
            parent_depth = p_node.depth;
            p_node.children.push(child);
            sibling_id
        };

        let c_node = self.get_treenode_mut(child)?;
        c_node.parent = Some(parent);
        c_node.previous_sibling = sibling_id;

        let depth_shift = parent_depth - c_node.depth + 1;
        let subtree: Vec<NodeId> = self.iter_subtree(child)?.collect();
        for c in subtree {
            self.get_treenode_mut(c)?.depth += depth_shift;
        }

        if let Some(sibling) = sibling_id {
            self.get_treenode_mut(sibling)?.next_sibling = Some(child);
        }

        Ok(())
    }

    /// Returns an iterator over the subtree starting with `head`. Implemented non-recursively
    pub fn iter_subtree(&self, head: NodeId) -> Result<TreeIterator<T>, TreeError> {
        TreeIterator::iter_subtree(self, head)
    }

    /// Returns a vector of children for a node
    pub fn get_children(&self, parent: NodeId) -> Result<&Vec<NodeId>, TreeError> {
        Ok(&self.get_treenode(parent)?.children)
    }

    /// Get the tree's root or error if there is none
    pub fn get_root(&self) -> Result<NodeId, TreeError> {
        match self.root {
            Some(id) => Ok(id),
            None => Err(TreeError::RootNotFound),
        }
    }

    /// If `child` has a parent, return its parent.
    pub fn get_parent(&self, child: NodeId) -> Result<NodeId, TreeError> {
        self.get_treenode(child)?
            .parent
            .ok_or(TreeError::ParentNotFound { child_id: child })
    }

    /// If `parent` has only one child, return that child. Otherwise, return an error
    pub fn get_only_child(&self, parent: NodeId) -> Result<NodeId, TreeError> {
        let children = self.get_children(parent)?;
        if children.len() == 1 {
            Ok(*children.first().unwrap())
        } else {
            Err(TreeError::ExpectedOnlyChild {
                parent_id: parent,
                child_num: children.len(),
            })
        }
    }

    /// If `parent` has children, return the first child. Otherwise, return an error
    pub fn get_first_child(&self, parent: NodeId) -> Result<NodeId, TreeError> {
        match self.get_children(parent)?.first() {
            Some(id) => Ok(*id),
            None => Err(TreeError::ChildNotFound { parent_id: parent }),
        }
    }

    /// If `parent` has children, return the last child. Otherwise, return an error
    pub fn get_last_child(&self, parent: NodeId) -> Result<NodeId, TreeError> {
        match self.get_children(parent)?.last() {
            Some(id) => Ok(*id),
            None => Err(TreeError::ChildNotFound { parent_id: parent }),
        }
    }

    /// Find the first child of a node for which `f` evaluates to `true` or `None` if all evaluate to `false`
    pub fn find_child(
        &self,
        parent: NodeId,
        f: &dyn Fn(NodeId, &T) -> bool,
    ) -> Result<Option<NodeId>, TreeError> {
        for child in self.get_children(parent)? {
            if f(*child, self.get_node(*child)?) {
                return Ok(Some(*child));
            }
        }
        Ok(None)
    }

    /// Return whether or not a child for which `f` evaluates to `true` exists
    pub fn child_exists(
        &self,
        parent: NodeId,
        f: &dyn Fn(NodeId, &T) -> bool,
    ) -> Result<bool, TreeError> {
        Ok(self.find_child(parent, f)?.is_some())
    }

    /// Get a reference to the `Node` specified by `id`
    fn get_treenode(&self, id: NodeId) -> Result<&Node<T>, TreeError> {
        if let Some(node) = self.nodes.get(&id) {
            Ok(node)
        } else {
            Err(TreeError::NodeNotFound { node_id: id })
        }
    }

    /// Get a mutable reference to the `Node` specified by `id`
    fn get_treenode_mut(&mut self, id: NodeId) -> Result<&mut Node<T>, TreeError> {
        if let Some(node) = self.nodes.get_mut(&id) {
            Ok(node)
        } else {
            Err(TreeError::NodeNotFound { node_id: id })
        }
    }
}

impl<'a, T> IntoIterator for &'a Tree<T> {
    type Item = NodeId;

    type IntoIter = TreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_subtree(self.root.unwrap()).unwrap()
    }
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for id in self {
            let node = self.get_treenode(id).unwrap();
            f.write_fmt(format_args!(
                "{}{:?}\n",
                "  ".repeat(node.depth.try_into().unwrap()),
                node.data
            ))?;
        }

        write!(f, "")
    }
}

/// Iterator that iterates over a tree in a depth-first fashion to get the ID's of every node
pub struct TreeIterator<'a, T> {
    tree: &'a Tree<T>,
    head: NodeId,
    cur_node: NodeId,
    finished: bool,
}

impl<'a, T> TreeIterator<'a, T> {
    fn iter_subtree(tree: &'a Tree<T>, head: NodeId) -> Result<TreeIterator<'a, T>, TreeError> {
        tree.get_node(head)?;
        Ok(TreeIterator {
            tree,
            head,
            cur_node: head,
            finished: false,
        })
    }
}

impl<'a, T> Iterator for TreeIterator<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let ret = self.cur_node;
        let mut node = self.tree.get_treenode(ret).unwrap();

        if !node.children.is_empty() {
            self.cur_node = *node.children.first().unwrap();
            return Some(ret);
        }

        loop {
            if let Some(sib) = node.next_sibling {
                self.cur_node = sib;
                return Some(ret);
            }
            if let Some(parent) = node.parent {
                node = self.tree.get_treenode(parent).unwrap();
                if parent == self.head {
                    self.finished = true;
                    return Some(ret);
                }
            } else {
                self.finished = true;
                return Some(ret);
            }
        }
    }
}

/// Generic node struct that stores a node's positional data (siblings, parents, etc.) and non-positional data of a generic type
struct Node<T> {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,

    /// Empty if node has no children
    children: Vec<NodeId>,

    /// The depth of this node in the tree. 0 if parent
    depth: u32,

    /// The non-positional data for this node
    data: T,
}

/// Struct to uniquely reference a node within a tree
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct NodeId {
    id: usize,
    tree_id: usize,
}
