/// Marker trait for EnumTag
pub trait EnumTag {
    /// Get the name and description of this enum
    fn enum_info(&self) -> (&'static str, &'static str);

    /// Get all possible enum variant names and descriptions
    fn enums() -> &'static [(&'static str, &'static str)];

    /// Build the enum from a name
    fn build_enum(name: String) -> Self;
}
