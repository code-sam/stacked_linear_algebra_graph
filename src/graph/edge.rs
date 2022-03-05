use super::vertex::VertexKey;

pub type EdgeType = String;
pub type EdgeTypeRef = str;

// pub enum Edge {
//     Directed(DirectedEdge),
// }

// TODO: add constructor with indices

// REVIEW: should an edge have a value, or even properties?
// TODO: use const generics
#[derive(Debug, Clone)]
pub struct DirectedEdge {
    // key: EdgeKey,
    edge_type: EdgeType,
    from_vertex: VertexKey,
    to_vertex: VertexKey,
}

impl DirectedEdge {
    pub fn new(
        // key: EdgeKey,
        from_vertex: VertexKey,
        to_vertex: VertexKey,
        edge_type: EdgeType,
    ) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            // key,
            edge_type,
            from_vertex,
            to_vertex,
        }
    }

    // pub fn key(&self) -> &EdgeKey {
    // &self.key
    // }
    // pub fn edge_type(&self) -> &EdgeType {
    //     &self.edge_type
    // }
    pub fn edge_type_ref(&self) -> &EdgeTypeRef {
        &self.edge_type.as_str()
    }
    pub fn originates_from_vertex(&self) -> &VertexKey {
        &self.from_vertex
    }
    pub fn points_to_vertex(&self) -> &VertexKey {
        &self.to_vertex
    }
}
