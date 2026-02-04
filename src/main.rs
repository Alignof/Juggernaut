//! The firmware for Juggernaut Gen3
//!
//! # Pin assign
//! | name                  | pin number |
//! |-----------------------|------------|
//! | Buzzer                | 28         |
//! | Shift Register SER    | 27         |
//! | Shift Register RCLK   | 26         |
//! | Shift Register SRCLK  | 22         |
//! | Start Switch          | 20         |
//! | Select Switch         | 21         |

#![no_std]
#![no_main]

mod display;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker, Timer};
use panic_halt as _;

use display::{ShiftRegister, SignalColor};

/// Represents the current global state of timer
struct TimerState {
    /// Remaining time in seconds.
    time_remain: i32,
    /// Current color of the signal LED.
    signal: SignalColor,
    /// Whether the countdown timer is paused.
    timer_stop: bool,
}

/// Global shared state protected by a Mutex for safe access across tasks.
static STATE: Mutex<ThreadModeRawMutex, TimerState> = Mutex::new(TimerState {
    time_remain: 0,
    signal: SignalColor::Yellow,
    timer_stop: true,
});

/// Task that handles the 1-second interval countdown and buzzer alerts.
///
/// # Arguments
/// * `buzzer` - The GPIO output pin connected to the buzzer.
#[embassy_executor::task]
async fn timer_task(mut buzzer: Output<'static>) {
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        ticker.next().await;
        let mut state = STATE.lock().await;

        if !state.timer_stop && state.time_remain > 0 {
            buzzer.set_high();
            state.time_remain -= 1;
            // Short beep logic could be added here
            Timer::after_millis(10).await;
            buzzer.set_low();
        }
    }
}

/// Task that manages the 7-segment display via shift registers.
/// Updates the display based on the current `GameState`.
///
/// # Arguments
/// * `shift_reg` - The shift register controller for the display.
#[embassy_executor::task]
async fn display_task(mut shift_reg: ShiftRegister) {
    loop {
        let (remain, signal) = {
            let state = STATE.lock().await;
            (state.time_remain, state.signal)
        };

        if remain <= 0 {
            // Logic for when time is up
            shift_reg.data_send(4, 0, signal).await;
            shift_reg.data_send(3, 0, signal).await;
            shift_reg.coron_send(signal).await;
            shift_reg.data_send(2, 0, signal).await;
            shift_reg.data_send(1, 0, signal).await;
        } else {
            let minutes = (remain / 60) as u8;
            let seconds = (remain % 60) as u8;

            shift_reg.data_send(4, minutes / 10, signal).await;
            shift_reg.data_send(3, minutes % 10, signal).await;
            shift_reg.coron_send(signal).await;
            shift_reg.data_send(2, seconds / 10, signal).await;
            shift_reg.data_send(1, seconds % 10, signal).await;
        }
    }
}

/// Main entry point for the Embassy executor.
/// Initializes peripherals and spawns background tasks.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Pin setup
    let shift_reg = ShiftRegister::new(
        Output::new(p.PIN_27, Level::Low), // SER
        Output::new(p.PIN_26, Level::Low), // RCLK
        Output::new(p.PIN_22, Level::Low), // SRCLK
    );
    let buzzer = Output::new(p.PIN_28, Level::Low);
    let start_sw = Input::new(p.PIN_20, Pull::Up);
    let select_sw = Input::new(p.PIN_21, Pull::Up);

    // Initial display
    spawner.spawn(display_task(shift_reg)).unwrap();
    spawner.spawn(timer_task(buzzer)).unwrap();

    let mut challenge_id = 0;
    let challenges_num = 2;

    // Challenge selection loop
    while start_sw.is_high() {
        if select_sw.is_low() {
            challenge_id = (challenge_id + 1) % challenges_num;
            // Update state to reflect selection if needed
            Timer::after_millis(300).await;
        }
        Timer::after_millis(10).await;
    }

    // Start game logic
    {
        let mut state = STATE.lock().await;
        state.time_remain = 300; // Example: 5 minutes
        state.timer_stop = false;
    }

    loop {
        // Main game logic or monitoring
        Timer::after_secs(1).await;
    }
}
