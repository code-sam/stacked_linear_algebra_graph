pub type VertexKey = String;
pub type VertexKeyRef = str;

// REVIEW: implementation implies Vertices from different graphs can be equal.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Vertex {
    key: VertexKey,
    value: VertexValue,
}

impl From<Vertex> for VertexKey {
    fn from(vertex: Vertex) -> Self {
        vertex.key_ref().to_owned()
    }
}

/// ```
/// # use cairn_knowledge_graph::graph::vertex::Vertex;
/// let vertex: Vertex = (String::from("Vertex key"), 1u8.into()).into();
/// assert_eq!(vertex.key_ref().to_owned(), String::from("Vertex key"));
/// assert_eq!(vertex.value(), 1u8.into());
/// ```
impl From<(VertexKey, VertexValue)> for Vertex {
    fn from(as_tuple: (VertexKey, VertexValue)) -> Self {
        Vertex::new(as_tuple.0, as_tuple.1)
    }
}

// REVIEW: if a StoredVertex takes about a much space as a VertexProperty,
// then the IndexDataStorage doesn't bring a benefit.
// pub(crate) struct StoredVertex {
//     store_index: StoreIndex, // a pointer has the same size as usize (~8byte)
//     vertex_property_type: VertexPropertyType, // enum of size isize (~8byte)
// }
// in total, StoredVertex takes 16 bytes, the same as a VertexProperty. Thereby,
// the use of an IndexedDataStore could only be efficient if data is stored with a larger size.

impl Vertex {
    pub fn new(key: VertexKey, value: VertexValue) -> Self {
        Self { key, value }
    }
    // pub fn key(&self) -> &VertexKey {
    //     &self.key
    // }
    pub fn key_ref(&self) -> &VertexKeyRef {
        &self.key
    }
    pub fn value_ref(&self) -> &VertexValue {
        &self.value
    }
    pub fn value(self) -> VertexValue {
        self.value
    }

    pub fn update_value(&mut self, new_value: VertexValue) {
        self.value = new_value;
    }
    pub fn update_key(&mut self, new_key: VertexKey) {
        self.key = new_key;
    }

    // TO REVIEW: converting Vertex to an enum would make the vertex immutable, but introduce runtime cost
    // It should not be possible to access/reach a deleted vertex
    // pub(crate) fn mark_as_removed(&mut self) {
    //     self.key = String::from("_deleted");
    //     self.value = VertexValue::None;
    // }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum VertexValueType {
    None, // TODO: is this useful, necessary, and a good idea?
    String,
    Boolean,
    Integer8Bit,
    Integer16Bit,
    Integer32Bit,
    Integer64Bit,
    Integer128Bit,
    UnsignedInteger8Bit,
    UnsignedInteger16Bit,
    UnsignedInteger32Bit,
    UnsignedInteger64Bit,
    UnsignedInteger128Bit,
    FloatingPoint32Bit,
    FloatingPoint64Bit,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum VertexValue {
    None, // REVIEW: is this useful, necessary, and a good idea?
    String(String),
    Boolean(bool),
    Integer8Bit(i8),
    Integer16Bit(i16),
    Integer32Bit(i32),
    Integer64Bit(i64),
    Integer128Bit(i128),
    UnsignedInteger8Bit(u8),
    UnsignedInteger16Bit(u16),
    UnsignedInteger32Bit(u32),
    UnsignedInteger64Bit(u64),
    UnsignedInteger128Bit(u128),
    FloatingPoint32Bit(f32),
    FloatingPoint64Bit(f64),
}

macro_rules! implement_from_type {
    ($value_type:ty, $vertex_property_enum_value:ident) => {
        impl From<$value_type> for VertexValue {
            fn from(item: $value_type) -> Self {
                VertexValue::$vertex_property_enum_value(item)
            }
        }
    };
}

implement_from_type!(String, String);
implement_from_type!(bool, Boolean);
implement_from_type!(i8, Integer8Bit);
implement_from_type!(i16, Integer16Bit);
implement_from_type!(i32, Integer32Bit);
implement_from_type!(i64, Integer64Bit);
implement_from_type!(i128, Integer128Bit);
implement_from_type!(u8, UnsignedInteger8Bit);
implement_from_type!(u16, UnsignedInteger16Bit);
implement_from_type!(u32, UnsignedInteger32Bit);
implement_from_type!(u64, UnsignedInteger64Bit);
implement_from_type!(u128, UnsignedInteger128Bit);
implement_from_type!(f32, FloatingPoint32Bit);
implement_from_type!(f64, FloatingPoint64Bit);
