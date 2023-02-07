use std::{thread, time::Duration};

use rppal::gpio::{
    Gpio,
    Level::{self, *},
};

const PIN_1: u8 = 17; // 11
const PIN_2: u8 = 27; // 13
const PIN_3: u8 = 22; // 15
const PIN_4: u8 = 10; // 19

const LEVELS: [[Level; 4]; 8] = [
    [High, Low, High, Low],
    [High, High, Low, Low],
    [Low, High, Low, Low],
    [Low, High, High, Low],
    [Low, Low, High, Low],
    [Low, Low, High, High],
    [Low, Low, Low, High],
    [High, Low, Low, High],
];

fn main() {
    let mut pin1 = Gpio::new().unwrap().get(PIN_1).unwrap().into_output();
    let mut pin2 = Gpio::new().unwrap().get(PIN_2).unwrap().into_output();
    let mut pin3 = Gpio::new().unwrap().get(PIN_3).unwrap().into_output();
    let mut pin4 = Gpio::new().unwrap().get(PIN_4).unwrap().into_output();

    let steps = 100;

    let mut reverse = false;

    loop {
        let levels: Vec<[Level; 4]> = if reverse {
            LEVELS.into_iter().rev().collect()
        } else {
            LEVELS.into_iter().collect()
        };

        for _ in 0..steps {
            // let level = LEVELS[step % 4];
            for level in &levels {
                pin1.write(level[0]);
                pin2.write(level[1]);
                pin3.write(level[2]);
                pin4.write(level[3]);
                thread::sleep(Duration::from_millis(10));
            }
        }

        reverse = !reverse;
        thread::sleep(Duration::from_millis(1000));
    }
}
