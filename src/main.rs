use std::{ error::Error, time::Duration };

use clap::{ Arg, Command };
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serialport::{ DataBits, Parity, StopBits };

fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 8 { (input & (1 << n)) != 0 } else { false }
}

#[derive(Debug, PartialEq, FromPrimitive)]
enum Mode {
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
struct SevenSegment {
    segments: [bool; 7],
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
struct MeterState {
    mode: Mode,
    seven_segments: [SevenSegment; 4],
    dot_positions: [bool; 3],
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

fn main() {
    let matches = Command::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
                .use_value_delimiter(false)
                .required(true)
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to connect at")
                .use_value_delimiter(false)
                .required(true)
                .validator(valid_baud)
        )
        .get_matches();

    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();

    let port = serialport
        ::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: [u8; 8] = [0; 8];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read_exact(&mut serial_buf) {
                    Ok(_t) => {
                        let meter_state = MeterState::new(serial_buf).unwrap();
                        if true {
                            print!("{}", meter_state.seven_segments[0].to_string());
                            if meter_state.dot_positions[0] {
                                print!(".");
                            }
                            print!("{}", meter_state.seven_segments[1].to_string());
                            if meter_state.dot_positions[1] {
                                print!(".");
                            }
                            print!("{}", meter_state.seven_segments[2].to_string());
                            if meter_state.dot_positions[2] {
                                print!(".");
                            }
                            print!("{}", meter_state.seven_segments[3].to_string());

                            print!("\n");
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

fn valid_baud(val: &str) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
