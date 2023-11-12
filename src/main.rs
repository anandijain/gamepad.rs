use enigo::*;
use gilrs::{ev::filter::Repeat, Axis, Button, Event, EventType, Gilrs};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();

    let gps = gilrs.gamepads();
    println!("{:?} gamepad(s) found", gps.collect::<Vec<_>>());
    println!("screen dimensions: {:?}", enigo.main_display_size());
    println!("mouse location: {:?}", enigo.mouse_location());

    let mut gps = gilrs.gamepads();
    // Iterate over all connected gamepads
    for (_id, gamepad) in gps {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    // Repeat

    // let ( gid, gp) = gps.next().unwrap();
    let mut active_gamepad = None;
    // enigo::MouseControllable::
    let mut button_to_key = HashMap::new();
    button_to_key.insert(Button::North, Key::Escape);
    button_to_key.insert(Button::East, Key::Space);
    // button_to_key.insert(Button::South, MouseButton::Right); // how can i get this to work
    button_to_key.insert(Button::LeftTrigger, Key::Shift);
    button_to_key.insert(Button::DPadUp, Key::UpArrow);
    button_to_key.insert(Button::DPadDown, Key::DownArrow);
    button_to_key.insert(Button::DPadLeft, Key::LeftArrow);
    button_to_key.insert(Button::DPadRight, Key::RightArrow);
    button_to_key.insert(Button::RightTrigger, Key::CapsLock);
    button_to_key.insert(Button::LeftTrigger2, Key::Alt);
    button_to_key.insert(Button::LeftTrigger, Key::Shift);
    button_to_key.insert(Button::RightTrigger2, Key::Tab);
    button_to_key.insert(Button::RightThumb, Key::Return);
    button_to_key.insert(Button::Start, Key::Meta);
    // button_to_key.insert(Button::Lef,   Key::F);

    let pause_btns = vec![
        // Button::RightTrigger,
        // Button::RightTrigger2,
        // Button::LeftTrigger,
        // Button::LeftTrigger2,
        // Button::Start
        Button::LeftThumb,
    ];

    let mut last_lefty = 0.;
    let mut last_leftx = 0.;
    let mut last_righty = 0.;
    let mut last_rightx = 0.;

    let mouse_ratio = 25.0;
    let scroll_ratio = 3.0;

    let mut dx = (mouse_ratio * last_leftx) as i32;
    let mut dy = -1 * (mouse_ratio * last_lefty) as i32;
    let mut dsx = (scroll_ratio * last_rightx) as i32;
    let mut dsy = -1 * (scroll_ratio * last_righty) as i32;

    let mut is_paused = false;
    let mut last_pause_time = Instant::now();
    let debounce_duration = Duration::from_millis(500); // 500 milliseconds debounce period

    loop {
        // Examine new events
        // probably want a thresholded guard
        if !is_paused && (dx.abs() > 1 || dy.abs() > 1) {
            println!("dx: {}, dy: {}", dx, dy);

            enigo.mouse_move_relative(dx, dy);
            std::thread::sleep(std::time::Duration::from_micros(1));
        }
        if !is_paused && (dsx.abs() > 1 || dsy.abs() > 1) {
            dsx = (scroll_ratio * last_rightx) as i32;
            dsy = -1 * (scroll_ratio * last_righty) as i32;

            enigo.mouse_scroll_x(dsx);
            enigo.mouse_scroll_y(dsy);
            std::thread::sleep(std::time::Duration::from_micros(1));
        }

        while let Some(Event { id, event, time }) = gilrs.next_event() {
            gilrs.inc();
            let c = gilrs.counter();
            // if c > 500 {
            //     return;
            // }
            println!("{c}: {:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
            if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                if pause_btns.iter().all(|btn| gamepad.is_pressed(*btn)) {
                    if last_pause_time.elapsed() > debounce_duration {
                        is_paused = !is_paused;
                        println!("is_paused: {}", is_paused);
                        last_pause_time = Instant::now(); // Update the timestamp
                    }
                }
            }
            if is_paused {
                continue;
            }

            match event {
                EventType::ButtonPressed(Button::Select, _) => {
                    enigo.key_down(Key::Meta);
                    enigo.key_down(Key::H);
                    enigo.key_up(Key::Meta);
                    enigo.key_up(Key::H);
                }

                EventType::ButtonPressed(Button::South, _) => {
                    enigo.mouse_down(MouseButton::Left);
                }

                EventType::ButtonReleased(Button::South, _) => {
                    enigo.mouse_up(MouseButton::Left);
                }

                EventType::AxisChanged(stick, val, _) => match stick {
                    Axis::LeftStickY => {
                        last_lefty = val;
                        dy = -1 * (mouse_ratio * last_lefty) as i32;
                    }
                    Axis::LeftStickX => {
                        last_leftx = val;
                        dx = (mouse_ratio * last_leftx) as i32;
                    }
                    Axis::RightStickY => {
                        last_righty = val;
                        dsy = -1 * (scroll_ratio * last_righty) as i32;
                    }
                    Axis::RightStickX => {
                        last_rightx = val;
                        dsx = (scroll_ratio * last_rightx) as i32;
                    }
                    _ => {}
                },

                EventType::ButtonPressed(button, _) => {
                    if let Some(&key) = button_to_key.get(&button) {
                        enigo.key_down(key);
                    }
                }
                EventType::ButtonReleased(button, _) => {
                    if let Some(&key) = button_to_key.get(&button) {
                        enigo.key_up(key);
                    }
                }
                _ => {}
            }
        }
    }
}
