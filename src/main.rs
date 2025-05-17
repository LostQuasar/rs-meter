#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::Duration;
use egui::{ frame, global_theme_preference_buttons, style::Selection, Color32, Frame, Stroke, Style, Theme };
use env_logger::fmt::style::Color;
use serialport::{ DataBits, Parity, SerialPort, StopBits };
pub mod meter;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "RS Multimeter",
        options,
        Box::new(|cc| {
            
        Ok(Box::new(MyApp::new(cc)))
        })
    )
}

fn use_custom_them(style: &mut Style) {
    style.visuals.panel_fill = Color32::from_rgb(153, 151, 145)
}
struct MyApp {
    port: Option<Box<dyn SerialPort + 'static>>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(Theme::Light);
        cc.egui_ctx.style_mut_of(Theme::Light, use_custom_them);
        Self {
            port: None
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel
            ::default()
            .show(ctx, |ui| {
                ui.heading("My egui Application");
            });
    }
}

fn setup_port(path: &str) -> Result<Box<dyn SerialPort + 'static>, serialport::Error> {
    serialport
        ::new(path, 4800)
        .timeout(Duration::from_millis(10))
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open()
}
fn read_port(mut port: Box<dyn SerialPort + 'static>) {
    let mut serial_buf: [u8; 8] = [0; 8];
    loop {
        match port.read_exact(&mut serial_buf) {
            Ok(_t) => {
                let meter_state = meter::MeterState::new(serial_buf).unwrap();
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
