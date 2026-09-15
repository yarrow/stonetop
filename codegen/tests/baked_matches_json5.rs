//! The baked content equals the json5 content. For every Move, Background, Special Possession,
//! and Backstory of every playbook, for every playbook itself, and for every gizmo in the gear
//! file, `key.fixed_part()` is the item freshly converted from the json5 and carries its own
//! key; the one Setting overview is `setting_overview()`. Together these tests cover the
//! conversion, the baking, the checked-in file, and the match in `fixed_part()`, so they need
//! `stonetop` built with `ssr` (a dev-dependency here).

use codegen::key::Key;
use codegen::{json5_gear, json5_playbook, json5_setting_overview, playbook_names};
use stonetop::fixed::setting_overview;

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

#[test]
fn every_background_bakes_to_itself() {
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        for background in &playbook.backgrounds {
            let key = background.key();
            let fresh = background.to_fixed();
            assert_eq!(
                key.fixed_part(),
                &fresh,
                "{key}: baked Background differs from {name}'s json5"
            );
            assert_eq!(key.fixed_part().key, key, "{key}: baked Background carries the wrong key");
        }
    }
}

#[test]
fn every_special_possession_bakes_to_itself() {
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        for possession in &playbook.special_possessions.options {
            let key = possession.key();
            let fresh = possession.to_fixed();
            assert_eq!(
                key.fixed_part(),
                &fresh,
                "{key}: baked Special Possession differs from {name}'s json5"
            );
            assert_eq!(
                key.fixed_part().key,
                key,
                "{key}: baked Special Possession carries the wrong key"
            );
        }
    }
}

#[test]
fn every_backstory_bakes_to_itself() {
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        for backstory in &playbook.backstory {
            let key = backstory.key();
            let fresh = backstory.to_fixed();
            assert_eq!(
                key.fixed_part(),
                &fresh,
                "{key}: baked Backstory differs from {name}'s json5"
            );
            assert_eq!(key.fixed_part().key, key, "{key}: baked Backstory carries the wrong key");
        }
    }
}

#[test]
fn every_playbook_bakes_to_itself() {
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        let key = playbook.key();
        let fresh = playbook.to_fixed();
        assert_eq!(key.fixed_part(), &fresh, "{key}: baked Playbook differs from {name}'s json5");
        assert_eq!(key.fixed_part().key, key, "{key}: baked Playbook carries the wrong key");
    }
}

#[test]
fn every_gizmo_bakes_to_itself() {
    let gear = json5_gear().unwrap_or_else(|e| panic!("{e:#}"));
    for gizmo in gear.gizmos() {
        let key = gizmo.key();
        let fresh = gizmo.to_fixed();
        assert_eq!(key.fixed_part(), &fresh, "{key}: baked gizmo differs from gear.json5");
        assert_eq!(key.fixed_part().key, key, "{key}: baked gizmo carries the wrong key");
    }
}

#[test]
fn the_setting_overview_bakes_to_itself() {
    let overview = json5_setting_overview().unwrap_or_else(|e| panic!("{e:#}"));
    let fresh = overview.to_fixed();
    assert_eq!(setting_overview(), &fresh, "baked Setting overview differs from its json5");
}
