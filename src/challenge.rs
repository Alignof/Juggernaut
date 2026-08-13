//! Competition-part (giver) challenge definitions for Juggernaut Gen3.
//!
//! The control part (`main`) drives the display, timer and buzzer. The giver
//! writes the actual puzzle here: each challenge inspects the competition
//! wires and decides whether the device has been defused ([`Outcome::Succeeded`])
//! or detonated ([`Outcome::Failed`]).
//!
//! To add a challenge, bump [`CHALLENGES_NUM`], add a branch to [`evaluate`],
//! and implement its polling function following the examples below.

use embassy_rp::gpio::Input;

/// Number of selectable challenges. The Select switch cycles `0..CHALLENGES_NUM`.
pub const CHALLENGES_NUM: u8 = 2;

/// The verdict for a challenge once the game has been decided.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The device was defused within the time limit.
    Succeeded,
    /// The device was detonated by an illegal operation.
    Failed,
}

/// The input wires exposed to the competition part (the giver's breadboard).
///
/// Each wire is configured as a pull-up input, so an intact wire (tied to GND)
/// reads low and a cut wire reads high.
pub struct Wires {
    /// Red wire.
    pub red: Input<'static>,
    /// Blue wire.
    pub blue: Input<'static>,
}

/// Poll the currently selected challenge once.
///
/// Returns `Some(Outcome)` when the game is decided, or `None` while it is
/// still in progress.
///
/// # Arguments
/// * `id` - The selected challenge index (`0..CHALLENGES_NUM`).
/// * `wires` - The competition wires to inspect.
pub fn evaluate(id: u8, wires: &Wires) -> Option<Outcome> {
    match id {
        0 => red_or_blue(wires),
        1 => red_or_blue_2(wires),
        _ => None,
    }
}

//=============================================================================
//  START of giver code
//=============================================================================

/// Challenge 0: cut the **red** wire to defuse; the blue wire detonates.
fn red_or_blue(wires: &Wires) -> Option<Outcome> {
    if wires.red.is_high() {
        return Some(Outcome::Succeeded);
    }
    if wires.blue.is_high() {
        return Some(Outcome::Failed);
    }
    None
}

/// Challenge 1: cut the **blue** wire to defuse; the red wire detonates.
fn red_or_blue_2(wires: &Wires) -> Option<Outcome> {
    if wires.blue.is_high() {
        return Some(Outcome::Succeeded);
    }
    if wires.red.is_high() {
        return Some(Outcome::Failed);
    }
    None
}

//=============================================================================
//  END of giver code
//=============================================================================
