//! The firmware for Juggernaut Gen3
//!
//! # Pin assign
//!
//! `GPIO` is the RP2350 GPIO number used in code; `header` is the Pico module's
//! physical pin number as wired in the gen3 schematic. They are NOT the same.
//!
//! | name                   | GPIO  | header pin |
//! |------------------------|-------|------------|
//! | Shift Register SER     | GP21  | 27         |
//! | Shift Register RCLK    | GP20  | 26         |
//! | Shift Register SRCLK   | GP19  | 25         |
//! | Buzzer (via Q5)        | GP16  | 21         |
//! | Start Switch  (SW3)    | GP17  | 22         |
//! | Select Switch (SW2)    | GP18  | 24         |
//! | Red Wire  (competition)| GP10  | 14         |
//! | Blue Wire (competition)| GP13  | 17         |

#![no_std]
#![no_main]

mod challenge;
mod display;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Instant, Timer};
use panic_halt as _;

use challenge::{CHALLENGES_NUM, Outcome, Wires};
use display::{DIGIT_NONE, ShiftRegister, SignalColor};

/// Represents the current global state shown on the display.
struct TimerState {
    /// Remaining time in seconds.
    time_remain: i32,
    /// Current color of the signal LED.
    signal: SignalColor,
    /// Whether the device is still in the challenge-selection phase.
    selecting: bool,
    /// The challenge index currently selected.
    challenge_id: u8,
}

/// Global shared state protected by a Mutex for safe access across tasks.
static STATE: Mutex<ThreadModeRawMutex, TimerState> = Mutex::new(TimerState {
    time_remain: 0,
    signal: SignalColor::Yellow,
    selecting: true,
    challenge_id: 0,
});

/// The way a game round can end.
enum GameEnd {
    /// The device was defused: signal green, timer frozen.
    Succeeded,
    /// The device detonated or ran out of time: signal red, buzzer alarm.
    Failed,
}

/// Task that continuously refreshes the multiplexed 7-segment display and the
/// signal LED based on the current [`STATE`].
///
/// # Arguments
/// * `shift_reg` - The shift register controller for the display.
#[embassy_executor::task]
async fn display_task(mut shift_reg: ShiftRegister) {
    loop {
        let (selecting, challenge_id, remain, signal) = {
            let state = STATE.lock().await;
            (
                state.selecting,
                state.challenge_id,
                state.time_remain,
                state.signal,
            )
        };

        if selecting {
            // Blank every digit, showing only the challenge id on the rightmost.
            shift_reg.data_send(4, DIGIT_NONE, false, signal).await;
            shift_reg.data_send(3, DIGIT_NONE, false, signal).await;
            shift_reg.data_send(2, DIGIT_NONE, false, signal).await;
            shift_reg.data_send(1, challenge_id, false, signal).await;
        } else {
            let remain = remain.max(0);
            let minutes = (remain / 60) as u8;
            let seconds = (remain % 60) as u8;

            // The center colon has no dedicated pin: its top dot is internally
            // tied to DP+DIG3 and its bottom dot to DP+DIG4, so asserting DP on
            // the two seconds digits lights the colon.
            shift_reg.data_send(4, minutes / 10, false, signal).await;
            shift_reg.data_send(3, minutes % 10, false, signal).await;
            shift_reg.data_send(2, seconds / 10, true, signal).await;
            shift_reg.data_send(1, seconds % 10, true, signal).await;
        }
    }
}

/// Runs a single challenge round: counts the timer down, beeps once per second,
/// and polls the selected challenge until the game is decided.
///
/// # Arguments
/// * `id` - The selected challenge index.
/// * `buzzer` - The GPIO output connected to the buzzer.
/// * `wires` - The competition wires inspected by the challenge.
/// * `time_limit` - The round length in seconds.
async fn run_game(
    id: u8,
    buzzer: &mut Output<'static>,
    wires: &Wires,
    time_limit: i32,
) -> GameEnd {
    let start = Instant::now();
    let mut last_second = time_limit;
    loop {
        let remain = time_limit - start.elapsed().as_secs() as i32;

        // Update the shared clock and emit a short beep on every new second.
        if remain != last_second {
            last_second = remain;
            STATE.lock().await.time_remain = remain;
            if remain > 0 {
                buzzer.set_high();
                Timer::after_millis(10).await;
                buzzer.set_low();
            }
        }

        // Time up counts as a failed defusal.
        if remain <= 0 {
            return GameEnd::Failed;
        }

        match challenge::evaluate(id, wires) {
            Some(Outcome::Succeeded) => return GameEnd::Succeeded,
            Some(Outcome::Failed) => return GameEnd::Failed,
            None => {}
        }

        Timer::after_millis(20).await;
    }
}

/// Main entry point for the Embassy executor.
/// Initializes peripherals, runs challenge selection, then the game round.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Control-part pin setup.
    // NOTE: these are RP2350 GPIO numbers, which differ from the Pico's
    // physical header pin numbers used in the gen3 schematic. See the pin
    // table at the top of this file for the header-pin <-> GPIO mapping.
    let shift_reg = ShiftRegister::new(
        Output::new(p.PIN_21, Level::Low), // SER   (header pin 27)
        Output::new(p.PIN_20, Level::Low), // RCLK  (header pin 26)
        Output::new(p.PIN_19, Level::Low), // SRCLK (header pin 25)
    );
    let mut buzzer = Output::new(p.PIN_16, Level::Low); // header pin 21
    let start_sw = Input::new(p.PIN_17, Pull::Up); // SW3, header pin 22
    let select_sw = Input::new(p.PIN_18, Pull::Up); // SW2, header pin 24

    // Competition-part wires (pull-up: cut wire reads high).
    let wires = Wires {
        red: Input::new(p.PIN_10, Pull::Up),
        blue: Input::new(p.PIN_13, Pull::Up),
    };

    spawner.spawn(display_task(shift_reg)).unwrap();

    // Challenge selection: cycle with Select, confirm with Start.
    let mut challenge_id = 0;
    while start_sw.is_high() {
        if select_sw.is_low() {
            challenge_id = (challenge_id + 1) % CHALLENGES_NUM;
            STATE.lock().await.challenge_id = challenge_id;
            // Debounce / hold so a single press advances once.
            Timer::after_millis(300).await;
        }
        Timer::after_millis(10).await;
    }

    // Start the round.
    let time_limit = 300; // 5 minutes
    {
        let mut state = STATE.lock().await;
        state.selecting = false;
        state.signal = SignalColor::Yellow;
        state.time_remain = time_limit;
    }

    match run_game(challenge_id, &mut buzzer, &wires, time_limit).await {
        GameEnd::Succeeded => {
            // Defused: freeze the clock and light the signal green.
            STATE.lock().await.signal = SignalColor::Green;
        }
        GameEnd::Failed => {
            // Detonated or timed out: red signal and a solid alarm until the
            // Start switch is pressed to acknowledge.
            STATE.lock().await.signal = SignalColor::Red;
            buzzer.set_high();
            while start_sw.is_high() {
                Timer::after_millis(10).await;
            }
            buzzer.set_low();
        }
    }

    // Halt: the display task keeps showing the final state.
    loop {
        Timer::after_secs(3600).await;
    }
}
