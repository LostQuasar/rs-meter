use std::error::Error;

use num::FromPrimitive;
use num_derive::FromPrimitive;

pub(crate) fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 8 { (input & (1 << n)) != 0 } else { false }
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub(crate) enum Mode {
    DcVoltage = 0x0,
    AcVoltage = 0x1,
    DcMicroAmpere = 0x2,
    DcMilliAmpere = 0x3,
    DcAmpere = 0x4,
    AcMicroAmpere = 0x5,
    AcMilliAmpere = 0x6,
    AcAmpere = 0x7,
    Resistance = 0x8,
    Capacitance = 0x9,
    FrequencySeconds = 0x10,
    FrequencyHz = 0xa,
    FrequencyPercent = 0xd,
    Diode = 0x13,
    Continuinty = 0x14,
    CurrentGain = 0x15,
    Logic = 0x16,
    PowerLevel = 0x17,
    Tempature = 0x19,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SevenSegment {
    pub(crate) segments: [bool; 7],
}

impl SevenSegment {
    pub fn to_string(self) -> char {
        match self.segments {
            [true, true, true, true, true, true, false] => '0',
            [false, true, true, false, false, false, false] => '1',
            [true, true, false, true, true, false, true] => '2',
            [true, true, true, true, false, false, true] => '3',
            [false, true, true, false, false, true, true] => '4',
            [true, false, true, true, false, true, true] => '5',
            [true, false, true, true, true, true, true] => '6',
            [true, true, true, false, false, false, false] => '7',
            [true, true, true, true, true, true, true] => '8',
            [true, true, true, true, false, true, true] => '9',
            [true, true, false, false, true, true, true] => 'P',
            [true, false, false, true, true, true, true] => 'E',
            [false, false, true, false, true, false, true] => 'N',
            [true, false, false, false, true, true, true] => 'F',
            [true, false, false, true, true, true, false] => 'C',
            [false, false, false, true, true, true, false] => 'L',
            [false, true, true, false, true, true, true] => 'H',
            [false, false, true, false, true, true, true] => 'H',
            [false, false, false, false, true, false, true] => 'R',
            [false, false, false, true, true, true, true] => 'T',
            [false, false, false, false, true, true, false] => 'I',
            [false, false, false, false, false, false, true] => '-',
            [false, false, false, false, false, false, false] => ' ',
            _ => '?',
        }
    }
}

#[derive(Debug)]
pub(crate) struct MeterState {
    pub(crate) mode: Mode,
    pub(crate) seven_segments: [SevenSegment; 4],
    pub(crate) dot_positions: [bool; 3],
}

impl MeterState {
    pub fn new(data: [u8; 8]) -> Result<Self, Box<dyn Error>> {
        let mode = Mode::from_u8(data[0]);
        if mode.is_none() {
            return Err(format!("Mode {} not recognized", data[0]).into());
        }
        let seven_segments = [
            SevenSegment {
                segments: [
                    get_bit_at(data[6], 0),
                    get_bit_at(data[6], 4),
                    get_bit_at(data[6], 6),
                    get_bit_at(data[6], 7),
                    get_bit_at(data[6], 2),
                    get_bit_at(data[6], 1),
                    get_bit_at(data[6], 5),
                ],
            },
            SevenSegment {
                segments: [
                    get_bit_at(data[5], 0),
                    get_bit_at(data[5], 4),
                    get_bit_at(data[5], 6),
                    get_bit_at(data[5], 7),
                    get_bit_at(data[5], 2),
                    get_bit_at(data[5], 1),
                    get_bit_at(data[5], 5),
                ],
            },
            SevenSegment {
                segments: [
                    get_bit_at(data[4], 0),
                    get_bit_at(data[4], 4),
                    get_bit_at(data[4], 6),
                    get_bit_at(data[4], 7),
                    get_bit_at(data[4], 2),
                    get_bit_at(data[4], 1),
                    get_bit_at(data[4], 5),
                ],
            },
            SevenSegment {
                segments: [
                    get_bit_at(data[3], 0),
                    get_bit_at(data[3], 4),
                    get_bit_at(data[3], 6),
                    get_bit_at(data[3], 7),
                    get_bit_at(data[3], 2),
                    get_bit_at(data[3], 1),
                    get_bit_at(data[3], 5),
                ],
            },
        ];

        let dot_positions = [
            get_bit_at(data[5], 3),
            get_bit_at(data[4], 3),
            get_bit_at(data[3], 3),
        ];

        Ok(Self {
            mode: mode.unwrap(),
            seven_segments,
            dot_positions,
        })
    }
}
