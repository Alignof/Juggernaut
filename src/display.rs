//! Display control module for the Juggernaut Gen3
//!
//! This module provides abstractions for controlling 7-segment displays
//! and RGB signal LEDs using bit-banging via shift registers.

use embassy_rp::gpio::Output;
use embassy_time::Timer;

/// Total number of bits to be sent to the shift register.
const DATASIZE: usize = 16;
/// Internal index representing the colon (:) on the display.
const DIGIT_CORON: u8 = 10;
/// Internal index representing an empty display (all segments off).
const DIGIT_NONE: u8 = 11;

/// Represents the available colors for the signal indicator.
#[derive(Clone, Copy)]
pub enum SignalColor {
    /// Red: Failed
    Red,
    /// Yellow: Progress
    Yellow,
    /// Green: Succeessed
    Green,
}

/// Controller for the 74HC595.
///
/// Manages the Serial (SER), Register Clock (RCLK), and Shift Register Clock (SRCLK)
/// pins to drive the connected display and LEDs.
pub struct ShiftRegister {
    /// Serial data input pin.
    ser: Output<'static>,
    /// Storage register clock pin (Latch).
    rclk: Output<'static>,
    /// Shift register clock pin.
    srclk: Output<'static>,
}

impl ShiftRegister {
    /// Creates a new `ShiftRegister` instance.
    ///
    /// # Arguments
    /// * `ser` - GPIO output for serial data.
    /// * `rclk` - GPIO output for the register clock (latch).
    /// * `srclk` - GPIO output for the shift register clock.
    pub fn new(ser: Output<'static>, rclk: Output<'static>, srclk: Output<'static>) -> Self {
        ShiftRegister { ser, rclk, srclk }
    }

    /// Sends a 16-bit data packet to the shift register to update the display.
    ///
    /// The data packet consists of:
    /// - Digit selection bits
    /// - Signal color bits
    /// - 7-segment pattern bits
    ///
    /// ```
    ///                QH <---------- QA
    /// register Left  :a,b,c,d,e,f,g,#,
    /// register Right :                R,Y,G,4,3,2,1,DP
    /// ```
    ///
    /// # Arguments
    /// * `digit` - The target digit index (1-based or specific position).
    /// * `num` - The number pattern index (0-9, or special characters).
    /// * `rgb` - The color to be set for the signal indicator.
    pub async fn data_send(&mut self, digit: u8, num: u8, rgb: SignalColor) {
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

    /// Specifically updates the colon (:) separator on the display.
    ///
    /// # Arguments
    /// * `rgb` - The current signal color bits to maintain while updating the colon.
    pub async fn coron_send(&mut self, rgb: SignalColor) {
        self.data_send(5, DIGIT_CORON, rgb).await;
    }
}
