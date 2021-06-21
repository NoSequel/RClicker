
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

    pub key_listeners: Rc<RefCell<Vec<RefCell<KeyListener<ClickerData>>>>>
}

impl ClickerData {
    pub fn new() -> Self {
        Self {
            enabled: false,
            min_cps: 12,
            max_cps: 15,
            enigo: Enigo::new(),
            key_listeners: Rc::new(RefCell::new(vec![
                RefCell::new(
                    KeyListener {
                        key_code: 0x58,
                        callback: Box::new(| data | {
                            data.enabled = !data.enabled;
                            thread::sleep(Duration::from_millis(120));
                        })
                    }
                ),

                RefCell::new(
                    KeyListener {
                        key_code: 0x01,
                        callback: Box::new(| data | {
                            if data.enabled {
                                let current: u64 = rand::thread_rng().gen_range(data.min_cps..data.max_cps);
        
                                data.enigo.mouse_up(MouseButton::Left);
                                data.enigo.mouse_down(MouseButton::Left);
        
                                thread::sleep(Duration::from_millis(1000 / current));
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