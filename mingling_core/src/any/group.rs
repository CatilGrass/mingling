/// Used to mark a type with a unique enum ID, assisting dynamic dispatch
pub trait Groupped<Group> {
    /// Returns the specific enum value representing its ID within that enum
    fn member_id() -> Group;
}
