//! Help menu: In-App Tutorials. Al Majlis Effects has no upstream documentation, community,
//! website, source-code, feedback or sibling-app links.

use super::{CommandSpec, always};
use crate::cmd;

pub fn specs() -> Vec<CommandSpec> {
    vec![cmd!("help.inAppTutorials", "In-App Tutorials...", ["Help"], None, "{} → the Home screen's Learn tab", always, |s, p| super::frontend(
        s,
        "help.inAppTutorials",
        p
    ))]
}
