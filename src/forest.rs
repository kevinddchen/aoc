#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NodeId(usize);

struct Node<T> {
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    data: T,
}

/// Forest data structure using region-based memory management to keep borrowing simple.
///
/// # Example
/// ```
/// use advent_2022::forest::Forest;
/// // Create a tree with a root node and a child node
/// let mut tree: Forest<i32> = Forest::new();
/// let root = tree.new_node(0);
/// let child = tree.add_child(root, 1);
/// assert_eq!(tree.get_parent(child), Some(root));
/// assert_eq!(*tree.get_data(root), 0);
/// ```
pub struct Forest<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Forest<T> {
    /// Creates a new forest.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    /// Creates a new root node in the forest.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store in the node.
    pub fn new_node(&mut self, data: T) -> NodeId {
        let id = NodeId(self.nodes.len());
        let node = Node {
            parent: None,
            children: Vec::new(),
            data,
        };
        self.nodes.push(node);
        return id;
    }

    /// Adds a new child node to the given parent node, and returns the child node.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent node.
    /// * `data` - The data to store in the child node.
    pub fn add_child(&mut self, parent: NodeId, data: T) -> NodeId {
        let child = self.new_node(data);
        self.nodes[parent.0].children.push(child);
        self.nodes[child.0].parent = Some(parent);
        return child;
    }

    /// Returns the parent of a node. The parent of a root node is `None`.
    pub fn get_parent(&self, node: NodeId) -> Option<NodeId> {
        return self.nodes[node.0].parent;
    }

    /// Returns the list of children of a node.
    pub fn get_children(&self, node: NodeId) -> Vec<NodeId> {
        return self.nodes[node.0].children.clone();
    }

    /// Returns the data of a node
    pub fn get_data(&self, node: NodeId) -> &T {
        return &self.nodes[node.0].data;
    }

    /// Returns mutable reference to the data of a node
    pub fn get_data_mut(&mut self, node: NodeId) -> &mut T {
        return &mut self.nodes[node.0].data;
    }
}

#[test]
fn test_one_node() {
    let mut forest: Forest<String> = Forest::new();
    let s = String::from("root");
    let root = forest.new_node(s.clone());
    assert_eq!(forest.get_parent(root), None);
    assert_eq!(forest.get_children(root).len(), 0);
    assert_eq!(forest.get_data(root), &s);
}

#[test]
fn test_two_nodes() {
    let mut forest: Forest<String> = Forest::new();
    let root = forest.new_node(String::from("root"));
    let child = forest.add_child(root, String::from("child"));
    assert_eq!(forest.get_parent(root), None);
    assert_eq!(forest.get_children(root).len(), 1);
    assert!(forest.get_children(root).contains(&child));
    assert_eq!(forest.get_parent(child), Some(root));
    assert_eq!(forest.get_children(child).len(), 0);
}

#[test]
fn test_three_nodes() {
    let mut forest: Forest<String> = Forest::new();
    let root = forest.new_node(String::from("root"));
    let child1 = forest.add_child(root, String::from("child1"));
    let child2 = forest.add_child(root, String::from("child2"));
    assert_eq!(forest.get_children(root).len(), 2);
    assert!(forest.get_children(root).contains(&child1));
    assert!(forest.get_children(root).contains(&child2));
    assert_eq!(forest.get_parent(child1), Some(root));
    assert_eq!(forest.get_parent(child2), Some(root));
    assert_ne!(child1, child2);
}

#[test]
fn test_get_data_mut() {
    let mut forest: Forest<i32> = Forest::new();
    let root = forest.new_node(0);
    assert_eq!(*forest.get_data(root), 0);
    *forest.get_data_mut(root) = 1;
    assert_eq!(*forest.get_data(root), 1);
}
