
use std::cell::RefCell;
use std::rc::Rc;

use enigo::{
    Enigo,
    MouseButton,
    MouseControllable
};

use std::{
    time::Duration,
};

use user32::GetKeyState;

use std::thread;
use rand::Rng;

pub struct ClickerData {
    pub enabled: bool,
    
    pub enigo: Enigo,

    pub min_cps: u64,
    pub max_cps: u64,

    pub debounce_time: u64,

    pub jitter_intensity_horizontal: i32,
    pub jitter_intensity_vertical: i32,

    pub key_listeners: Rc<RefCell<Vec<RefCell<KeyListener<ClickerData>>>>>
}

impl ClickerData {
    pub fn new() -> Self {
        Self {
            enabled: false,

            jitter_intensity_horizontal: 12,
            jitter_intensity_vertical: 4,

            min_cps: 12,
            max_cps: 15,

            debounce_time: 0,

            enigo: Enigo::new(),
            key_listeners: Rc::new(RefCell::new(vec![
                RefCell::new(
                    KeyListener {
                        key_code: 0x58,
                        callback: Box::new(| data | {
                            data.enabled = !data.enabled;
                            thread::sleep(Duration::from_millis(200));
                        })
                    }
                ),

                RefCell::new(
                    KeyListener {
                        key_code: 0x01,
                        callback: Box::new(| data | {
                            if data.enabled {
                                let mut rand = rand::thread_rng();
                                let current: u64 = rand.gen_range(data.min_cps..data.max_cps);
        
                                data.enigo.mouse_up(MouseButton::Left);

                                if data.jitter_intensity_horizontal != 0 {
                                    data.enigo.mouse_move_relative(rand.gen_range(-data.jitter_intensity_horizontal..data.jitter_intensity_horizontal), 0);
                                }

                                if data.jitter_intensity_vertical != 0 {
                                    data.enigo.mouse_move_relative(0, rand.gen_range(-data.jitter_intensity_vertical..data.jitter_intensity_vertical));
                                }

                                thread::sleep(Duration::from_millis(data.debounce_time));

                                data.enigo.mouse_down(MouseButton::Left);
        
                                thread::sleep(Duration::from_millis((1000 / current) - data.debounce_time));
                            }
                        })
                    }
                )
            ]))
        }
    }

    pub fn handle_listeners(&mut self) {
        for element in self.key_listeners.clone().borrow_mut().iter() {
            element.borrow_mut().process_events(self);
        }
    }
}

pub struct KeyListener<T> {
    pub key_code: i32,
    pub callback: Box<dyn FnMut(&mut T)>
}


impl<T> KeyListener<T> {
    pub fn process_events(&mut self, data: &mut T) {
        unsafe {
            if GetKeyState(self.key_code) < 0 {
                (self.callback)(data);
            }
        }
    }
}