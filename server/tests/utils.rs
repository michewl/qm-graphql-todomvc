/// Check two arrays if any tag from the first is contained in the second.
pub fn has_tag_condition(tags: &[String], check_tags: &[String], include: bool) -> bool {
    if include {
        tags.iter().any(|t| check_tags.contains(t))
    } else {
        !tags.iter().any(|t| check_tags.contains(t))
    }
}
