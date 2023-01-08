use crate::{session::SessionInfo, Metadata, Report, Rule};

use if_chain::if_chain;
use macros::lint;
use rnix::{
    types::{Lambda, TypedNode, Pattern},
    NodeOrToken, SyntaxElement, SyntaxKind
};

/// ## What it does
/// Ensure that `lib` is the first in the pattern
///
/// ## Why is this bad?
/// Nixpkgs policy
///
/// ## Example
/// ```nix
/// { other, names, lib }:
/// ```
///
/// Put `lib` first
/// ```nix
/// { lib, other, names }:
/// ```

#[lint(
    name = "lib_first",
    note = "Lib should be the first in the pattern",
    code = 22,
    match_with = SyntaxKind::NODE_ROOT
)]
struct LibFirst;

fn let_present_not_first(pattern: &Pattern) -> bool {
    pattern.entries().enumerate().any(|(i, entry)| {
        i > 0 && entry.name().map(|x| x.node().text() == "lib") == Some(true)
    })
}

impl Rule for LibFirst {
    fn validate(&self, node: &SyntaxElement, _sess: &SessionInfo) -> Option<Report> {
        if_chain! {
            if let NodeOrToken::Node(node) = node;
            if let Some(node) = node.children().next();
            if let Some(lambda) = Lambda::cast(node.clone());
            if let Some(pattern) = lambda.arg();
            if let Some(pattern) = Pattern::cast(pattern.clone());
            if let_present_not_first(&pattern);
            then {
                let at = pattern.node().text_range();
                let message = "`lib` should be the first in the list";
                Some(
                    self.report()
                        .diagnostic(at, message),
                )
            } else {
                None
            }
        }
    }
}
