use crate::{session::SessionInfo, Metadata, Report, Rule, Suggestion};

use if_chain::if_chain;
use macros::lint;
use rnix::{
    types::{BinOp, BinOpKind, TypedNode, List},
    NodeOrToken, SyntaxElement, SyntaxKind, SyntaxNode,
};

/// ## What it does
/// Checks for concatenations to empty lists
///
/// ## Why is this bad?
/// Unnecessary code.
///
/// ## Example
/// ```nix
/// [] ++ something
/// ```
///
/// Remove the operation:
///
/// ```nix
/// something
/// ```
#[lint(
    name = "empty_list_concat",
    note = "Unnecessary empty list concatenation",
    code = 23,
    match_with = SyntaxKind::NODE_BIN_OP
)]
struct EmptyListConcat;

impl Rule for EmptyListConcat {
    fn validate(&self, node: &SyntaxElement, _sess: &SessionInfo) -> Option<Report> {
        if_chain! {
            if let NodeOrToken::Node(node) = node;
            if let Some(bin_expr) = BinOp::cast(node.clone());
            if let Some(lhs) = bin_expr.lhs();
            if let Some(rhs) = bin_expr.rhs();
            if let Some(op) = bin_expr.operator();
            if let BinOpKind::Concat = op;
            then {
                let at = node.text_range();
                let message = "Unnecessary empty list concatenation";
                if is_empty_array(&lhs) {
                    Some(self.report().suggest(at, message, Suggestion::new(at, rhs)))
                } else if is_empty_array(&rhs) {
                    Some(self.report().suggest(at, message, Suggestion::new(at, lhs)))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

fn is_empty_array(node: &SyntaxNode) -> bool {
    if_chain! {
        if let Some(list) = List::cast(node.clone());
        then {
            list.items().count() == 0
        } else {
            false
        }
    }
}
