//! The State half of the Fixed/State split: the player's marks, the only part that changes
//! during play and the only part persisted. So far only gizmos have a state type here.

use serde::{Deserialize, Serialize};

use crate::keys::{GizmoKey, PlaybookKey};

/// The address of one instance of a gizmo a character has: the playbook, the gizmo, and a
/// number that tells this instance from the character's other instances of the same gizmo
/// (four rushlights, two sets of extra arrows).
///
/// Within one playbook and gizmo, numbers only grow: the next is one more than the highest
/// ever used (see [`next_instance_number`]), and removing an instance leaves a gap that is never
/// refilled. So a form field or URL naming instance 2 still means instance 2 after the page
/// re-renders, however many instances have come and gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InstanceAddress {
    pub playbook: PlaybookKey,
    pub gizmo: GizmoKey,
    pub number: u32,
}

/// What a character's instance of a gizmo holds: its resource's fullness, and nothing else
/// yet. Whether the instance is carried, and which slots it marks, are the Inventory step's.
///
/// When an instance comes into being is a rule of the possession, not of this type: a kit of
/// one and each picked Weapon of War create instance 0 on selection, because the print puts
/// their circles on the playbook, and a multi-gizmo kit creates none, because its contents are
/// a menu. Availability is derived from the fixed content and the selected possessions, never
/// stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GizmoState {
    /// For a counted resource, how many are left unmarked; for a labelled one, the index of the
    /// current state in its labels, worst first. Zero for a gizmo with no resource.
    pub fullness: u8,
}

/// The number for a new instance, given the numbers of every instance the character has ever
/// had of that gizmo: one more than the highest, or 0 for the first.
pub fn next_instance_number(ever_used: impl IntoIterator<Item = u32>) -> u32 {
    ever_used.into_iter().max().map_or(0, |highest| highest + 1)
}

#[cfg(test)]
mod tests {
    use super::next_instance_number;

    #[test]
    fn the_next_number_is_one_past_the_highest_ever_used_and_a_gap_is_never_refilled() {
        assert_eq!(next_instance_number([]), 0);
        assert_eq!(next_instance_number([0]), 1);
        // Instance 1 of 0..=3 was removed: the gap stays, and the next is 4.
        assert_eq!(next_instance_number([0, 2, 3]), 4);
        // Only instance 3 remains of 0..=3: still 4, not 0 or 1.
        assert_eq!(next_instance_number([3]), 4);
    }
}
