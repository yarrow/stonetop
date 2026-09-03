//! The baked content equals the json5 content. For every Move of every playbook,
//! `key.fixed_part()` is the Move freshly converted from the json5 and carries its own key. This
//! one test covers the conversion, the baking, the checked-in file, and the match in
//! `fixed_part()`, so it needs `stonetop` built with `ssr` (a dev-dependency here).

use codegen::key::Key;
use codegen::{json5_playbook, playbook_names};

#[test]
fn every_move_bakes_to_itself() {
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        for a_move in playbook.moves() {
            let key = a_move.key();
            let fresh = a_move.to_fixed();
            assert_eq!(key.fixed_part(), &fresh, "{key}: baked Move differs from {name}'s json5");
            assert_eq!(key.fixed_part().key, key, "{key}: baked Move carries the wrong key");
        }
    }
}
