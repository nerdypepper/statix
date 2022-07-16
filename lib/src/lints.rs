use crate::lints;

lints! {
    bool_comparison,
    empty_let_in,
    manual_inherit,
    manual_inherit_from,
    legacy_let_syntax,
    collapsible_let_in,
    eta_reduction,
    useless_parens,
    // unquoted_splice,
    empty_pattern,
    redundant_pattern_bind,
    unquoted_uri,
    deprecated_is_null,
    empty_inherit,
    faster_groupby,
    faster_zipattrswith,
    deprecated_to_path,
    bool_simplification,
    useless_has_attr,
    repeated_keys,
}
