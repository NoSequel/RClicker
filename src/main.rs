extern crate user32;

use enigo::{
    Enigo,
    MouseButton,
    MouseControllable
};

use std::{
    io::{stdout, Write},
    time::Duration,
};

use user32::GetKeyState;
use std::thread;
use rand::Rng;

static mut MAX_CPS: u64 = 15;
static mut MIN_CPS: u64 = 13;
static mut ENABLED: bool = false;

fn main() {
    let mut enigo = Enigo::new();
    let mut random = rand::thread_rng();
    let mut stdout = stdout();

    let mut listener = KeyListener {
        key_code: 0x01,
        callback: || {
            unsafe {
                let current: u64 = random.gen_range(MIN_CPS..MAX_CPS);
    
                enigo.mouse_up(MouseButton::Left);
                enigo.mouse_down(MouseButton::Left);
    
                thread::sleep(Duration::from_millis(1000 / current));
            }
        }
    };

    unsafe {
        loop {
            print!(
                "\r{} {} {}",
                "Autoclicker", 
                " [Toggled]",
                format!(" [{}] [{}]", MAX_CPS, MIN_CPS)
            );

            stdout.flush().unwrap();

            if GetKeyState(0x58) < 0 {
                ENABLED = !ENABLED;
                thread::sleep(Duration::from_millis(125));
            }

            if GetKeyState(0x26) < 0 {
                MAX_CPS += 1;
                MIN_CPS += 1;

                thread::sleep(Duration::from_millis(125));
            }

            if GetKeyState(0x28) < 0{
                MAX_CPS -= 1;
                MIN_CPS -= 1;

                thread::sleep(Duration::from_millis(125));
            }

            if ENABLED && GetKeyState(listener.key_code) < 0 {
                listener.process_events();
            }
        }
    }
}

struct KeyListener<CB> where CB : FnMut() {
    key_code: i32,
    callback: CB
}

impl<CB> KeyListener<CB> where CB : FnMut() {
    fn process_events(&mut self) {
        (self.callback)();
    }
}