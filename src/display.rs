//! Display control module for the Juggernaut Gen3
//!
//! This module provides abstractions for controlling 7-segment displays
//! and RGB signal LEDs using bit-banging via shift registers.

use embassy_rp::gpio::Output;
use embassy_time::Timer;

/// Total number of bits to be sent to the shift register.
const DATASIZE: usize = 16;
/// Index representing an empty display cell (all segments off).
pub const DIGIT_NONE: u8 = 10;
/// Decimal-point segment bit within a digit's 8-bit pattern.
const SEG_DP: u16 = 0x80;

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

    /// Sends a 16-bit data packet to the shift register to light one digit.
    ///
    /// The display is multiplexed: this enables a single digit, drives its
    /// segments, and refreshes the signal LED. Bit layout of the 16-bit word,
    /// as wired on gen3 (U1 = high byte, U2 = low byte; `FJ5462AH`, common
    /// cathode, all signals active-high):
    ///
    /// ```text
    /// bit : 15   14   13   12   11   10   9    8    7   6 5 4 3 2 1 0
    /// use : DIG1 DIG2 DIG3 DIG4  --  Grn  Yel  Red  DP  g f e d c b a
    /// ```
    ///
    /// Digit index maps left-to-right: index 4 -> DIG1 (leftmost) ...
    /// index 1 -> DIG4 (rightmost), hence the digit-enable bit is `digit + 11`.
    ///
    /// # Arguments
    /// * `digit` - The digit index, 1 (rightmost) to 4 (leftmost).
    /// * `num` - The number pattern index (0-9, or [`DIGIT_NONE`]).
    /// * `dp` - Whether to also light this digit's decimal point.
    /// * `rgb` - The color to be set for the signal indicator.
    pub async fn data_send(&mut self, digit: u8, num: u8, dp: bool, rgb: SignalColor) {
        let seg: [u16; 11] = [
            0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f, // 0 - 9
            0x00, // none
        ];

        let mut pattern = seg[num as usize];
        if dp {
            pattern |= SEG_DP;
        }

        let data: u16 = (1 << (digit + 11)) | (1 << (rgb as u8 + 8)) | pattern;

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
