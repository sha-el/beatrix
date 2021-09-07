use crate::rdms::select::Select;

pub enum QueryType {
    Select(Select),
    Update,
    Alter,
    Delete,
}
