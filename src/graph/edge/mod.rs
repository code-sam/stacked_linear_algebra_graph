mod edge;

pub(crate) mod adjacency_matrix;

// re-export mod edge to reduce the depth of the public module tree.
// As long as mod edge remains the only public mod in this module,
// further decomposition of the tree is not useful.
pub use edge::*;
