use crate::{lookup_table::LookupID, math_type::Type};
pub enum Definition_Field {
    NoValue,
    ExistingType(LookupID<Type>),
    New,
}

pub struct Definition {
    pub name: String,
    pub input_1_type: Definition_Field,
    pub input_2_type: Definition_Field,
}
