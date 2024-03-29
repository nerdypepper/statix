use crate::{
    make,
    session::{SessionInfo, Version},
    Metadata, Report, Rule, Suggestion,
};

use if_chain::if_chain;
use macros::lint;
use rnix::{
    types::{Select, TypedNode},
    NodeOrToken, SyntaxElement, SyntaxKind,
};

/// ## What it does
/// Checks for `lib.zipAttrsWith`.
///
/// ## Why is this bad?
/// Nix 2.6 introduces `builtins.zipAttrsWith` which is faster and does
/// not require a lib import.
///
/// ## Example
///
/// ```nix
/// lib.zipAttrsWith (name: values: values) [ {a = "x";} {a = "y"; b = "z";} ]
/// # { a = ["x" "y"]; b = ["z"] }
/// ```
///
/// Replace `lib.zipAttrsWith` with `builtins.zipAttrsWith`:
///
/// ```nix
/// builtins.zipAttrsWith (name: values: values) [ {a = "x";} {a = "y"; b = "z";} ]
/// ```
#[lint(
    name = "faster_zipattrswith",
    note = "Found lib.zipAttrsWith",
    code = 16,
    match_with = SyntaxKind::NODE_SELECT
)]
struct FasterZipAttrsWith;

impl Rule for FasterZipAttrsWith {
    fn validate(&self, node: &SyntaxElement, sess: &SessionInfo) -> Option<Report> {
        let lint_version = "2.6".parse::<Version>().unwrap();
        if_chain! {
            if sess.version() >= &lint_version;
            if let NodeOrToken::Node(node) = node;
            if let Some(select_expr) = Select::cast(node.clone());
            if let Some(select_from) = select_expr.set();
            if let Some(zip_attrs_with) = select_expr.index();

            // a heuristic to lint on nixpkgs.lib.zipAttrsWith
            // and lib.zipAttrsWith and its variants
            if select_from.text() != "builtins";
            if zip_attrs_with.text() == "zipAttrsWith";

            then {
                let at = node.text_range();
                let replacement = {
                    let builtins = make::ident("builtins");
                    make::select(builtins.node(), &zip_attrs_with).node().clone()
                };
                let message = format!("Prefer `builtins.zipAttrsWith` over `{}.zipAttrsWith`", select_from);
                Some(
                    self.report()
                        .suggest(at, message, Suggestion::new(at, replacement)),
                )
            } else {
                None
            }
        }
    }
}
