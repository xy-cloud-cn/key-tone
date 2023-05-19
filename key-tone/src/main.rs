extern crate rodio;
extern crate device_query;
extern crate tinyfiledialogs;

use std::fs::{File, metadata};
use std::io::{Read, Cursor};
use rodio::{OutputStream, source::Source};
use device_query::{DeviceQuery, DeviceState, Keycode};
use tinyfiledialogs::message_box_ok;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let device_state = DeviceState::new();
    let mut prev_keys: Vec<Keycode> = Vec::new();
    if metadata("ding.wav").is_err() {
        message_box_ok("Error", "File not found", tinyfiledialogs::MessageBoxIcon::Error);
        return;
    }
    let mut file = File::open("ding.wav").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if !keys.is_empty() && prev_keys.is_empty() {
            let source = rodio::Decoder::new(Cursor::new(buffer.clone())).unwrap();
            stream_handle.play_raw(source.convert_samples()).unwrap();
        }
        prev_keys = keys;
    }
}
