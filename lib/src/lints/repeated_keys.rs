use crate::{session::SessionInfo, Metadata, Report, Rule};

use std::collections::BTreeMap;
use if_chain::if_chain;
use macros::lint;
use rnix::{
    types::{AttrSet, KeyValue, TypedNode},
    NodeOrToken, SyntaxElement, SyntaxKind
};

/// ## What it does
/// Prevent repeating keys.
///
/// ## Why is this bad?
/// It's bad for readability and introduces repetition.
///
/// ## Example
/// ```nix
/// {
///   foo.key1 = 1;
///   foo.key2 = 2;
///   foo.key3 = 3;
/// }
/// ```
///
/// Don't repeat.
/// ```nix
/// {
///   foo {
///     key1 = 1;
///     key2 = 2;
///     key3 = 3;
///   }
/// }
/// ```

#[lint(
    name = "repeated_keys",
    note = "Common key paths should not be repeated",
    code = 20,
    match_with = SyntaxKind::NODE_ATTR_SET
)]
struct RepeatedKeys;

fn path_counts(attr_set: &AttrSet) -> Vec<String> {
    let mut counts = BTreeMap::new();
    counts = attr_set.node().children().filter_map(|ref c| {
        KeyValue::cast(c.clone()).and_then(|kv| {
            let key = kv.key().unwrap();
            let mut path = key.path();
            let first_path_part = path.next().unwrap().to_string();
            if path.next().is_none() {
                None
            } else {
                Some(first_path_part)
            }
        })
    }).fold(counts, |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    counts.retain(|_, &mut v| v >= 3);
    counts.into_keys().collect()
}

impl Rule for RepeatedKeys {
    fn validate(&self, node: &SyntaxElement, _sess: &SessionInfo) -> Option<Report> {
        if_chain! {
            if let NodeOrToken::Node(node) = node;
            if let Some(attr_set) = AttrSet::cast(node.clone());
            then {
                let repeated = path_counts(&attr_set);
                if repeated.len() > 0 {
                    let at = node.text_range();
                    let mut report = self.report();
                    for attr in repeated {
                        let message = format!("Key `{}` used 3 or more times. Can be replaced with `{} {{ foo=...; bar=...; }}`", attr, attr);
                        report = report.diagnostic(at, message);
                    }
                    Some(report)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}