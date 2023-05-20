extern crate device_query;
extern crate rodio;
extern crate tinyfiledialogs;

use device_query::{DeviceQuery, DeviceState};
use rodio::{source::Source, OutputStream};
use std::fs::{metadata, File};
use std::io::{Cursor, Read};
use tinyfiledialogs::message_box_ok;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let device_state = DeviceState::new();

    if metadata("ding.wav").is_err() {
        message_box_ok(
            "Error",
            "File not found",
            tinyfiledialogs::MessageBoxIcon::Error,
        );
        return;
    }
    let mut file = File::open("ding.wav").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut prev_keys_len = 0;
    loop {
        let keys_len = device_state.get_keys().len();
        if keys_len > prev_keys_len {
            let source = rodio::Decoder::new(Cursor::new(buffer.clone())).unwrap();
            stream_handle.play_raw(source.convert_samples()).unwrap();
        }
        prev_keys_len = keys_len;
    }
}
