use struct_field_names_as_array::FieldNamesAsSlice;

pub trait FieldnamesAsStringVec {
    fn field_names_as_string_vec() -> Vec<String>;
}

impl<T> FieldnamesAsStringVec for T
where
    T: FieldNamesAsSlice,
{
    fn field_names_as_string_vec() -> Vec<String> {
        <Self as FieldNamesAsSlice>::FIELD_NAMES_AS_SLICE
            .iter()
            .map(|d| String::from(*d))
            .collect()
    }
}
