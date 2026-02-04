//! Juggernaut Gen3
//!
//! This code is a firmware for Juggernaut Gen3

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker, Timer};
use panic_halt as _;

const DATASIZE: usize = 16;
const DIGIT_CORON: u8 = 10;
const DIGIT_NONE: u8 = 11;

#[derive(Clone, Copy)]
enum SignalColor {
    Red,
    Yellow,
    Green,
}

struct GameState {
    time_remain: i32,
    signal: SignalColor,
    timer_stop: bool,
}

static STATE: Mutex<ThreadModeRawMutex, GameState> = Mutex::new(GameState {
    time_remain: 0,
    signal: SignalColor::Yellow,
    timer_stop: true,
});

struct ShiftRegister {
    ser: Output<'static>,
    rclk: Output<'static>,
    srclk: Output<'static>,
}

impl ShiftRegister {
    async fn data_send(&mut self, digit: u8, num: u8, rgb: SignalColor) {
        let seg = [
            0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f, // 0 - 9
            0x03, 0x00, // colon, none
        ];

        let data: u16 = (1 << (digit + 10)) | (1 << (rgb as u8 + 8)) | (seg[num as usize]);

        self.rclk.set_low();
        for i in 0..DATASIZE {
            if (data >> i) & 1 == 1 {
                self.ser.set_high();
            } else {
                self.ser.set_low();
            }
            self.srclk.set_low();
            self.srclk.set_high();
        }
        self.rclk.set_high();
        Timer::after_millis(1).await;
    }
}

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
            Timer::after_millis(100).await;
            buzzer.set_low();
        }
    }
}

#[embassy_executor::task]
async fn display_task(mut shift_reg: ShiftRegister) {
    loop {
        let (remain, signal) = {
            let state = STATE.lock().await;
            (state.time_remain, state.signal)
        };

        if remain <= 0 {
            // Logic for when time is up
            shift_reg.data_send(4, 0, signal).await; // Simplified for brevity
        } else {
            let minutes = (remain / 60) as u8;
            let seconds = (remain % 60) as u8;

            shift_reg.data_send(4, minutes / 10, signal).await;
            shift_reg.data_send(3, minutes % 10, signal).await;
            shift_reg.data_send(5, DIGIT_CORON, signal).await;
            shift_reg.data_send(2, seconds / 10, signal).await;
            shift_reg.data_send(1, seconds % 10, signal).await;
        }
    }
}

// --- Main ---
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Pin setup
    let shift_reg = ShiftRegister {
        ser: Output::new(p.PIN_27, Level::Low),
        rclk: Output::new(p.PIN_26, Level::Low),
        srclk: Output::new(p.PIN_25, Level::Low),
    };
    let buzzer = Output::new(p.PIN_12, Level::Low);
    let sys_sw = Input::new(p.PIN_10, Pull::Up);
    let select_sw = Input::new(p.PIN_11, Pull::Up);

    // Initial display
    spawner.spawn(display_task(shift_reg)).unwrap();
    spawner.spawn(timer_task(buzzer)).unwrap();

    let mut challenge_id = 0;
    let challenges_num = 2;

    // Challenge selection loop
    while sys_sw.is_high() {
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
