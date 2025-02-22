use crate::graph::indexing::EdgeTypeIndex;


pub(crate) struct SparseMatrixChangeset {
    deleted_value_mask: EdgeTypeIndex,
    added_value_mask: EdgeTypeIndex,
    element_value_delta: EdgeTypeIndex
}