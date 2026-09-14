use codegen::{json5_gear, json5_playbook};

fn loads_ok(playbook_name: &str) {
    if let Err(e) = json5_playbook(playbook_name) {
        panic!("Failed: {e:#?}");
    }
}
#[test]
fn blessed() {
    loads_ok("blessed");
}
#[test]
fn fox() {
    loads_ok("fox");
}
#[test]
fn heavy() {
    loads_ok("heavy");
}
#[test]
fn judge() {
    loads_ok("judge");
}
#[test]
fn lightbearer() {
    loads_ok("lightbearer");
}
#[test]
fn marshal() {
    loads_ok("marshal");
}
#[test]
fn ranger() {
    loads_ok("ranger");
}
#[test]
fn seeker() {
    loads_ok("seeker");
}
#[test]
fn would_be_hero() {
    loads_ok("would-be-hero");
}
#[test]
fn gear() {
    if let Err(e) = json5_gear() {
        panic!("Failed: {e:#?}");
    }
}
