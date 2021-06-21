mod clicker;
use clicker::ClickerData;

use coffee::{
    input::{
        mouse::{
            Mouse
        }
    },
    graphics::{
        Color,
        Frame,
        HorizontalAlignment,
        Window,
        WindowSettings,
        Shape,
        Mesh,
        Rectangle
    },

    load::{
        Task
    },

    Game,
    Result,
    Timer,
    ui::{
        Renderer,
        Element,
        Column,
        UserInterface,
        slider,
        Row,
        Text,
        Slider,
        Align,
        Justify,
    }
};

use std::ops::RangeInclusive;

fn main() -> Result<()> {
    <ClickerGUI as UserInterface>::run(WindowSettings {
        title: String::from("RClicker"),
        size: (768, 400),
        resizable: false,
        fullscreen: false,
        maximized: false
    })
}

enum Message {
    MinClicksChanged(f32),
    MaxClicksChanged(f32),

    HorizontalJitterIntensityChanged(f32),
    VerticalJitterIntensityChanged(f32),
}

struct ClickerGUI {
    data: ClickerData,

    min_cps_slider: slider::State,
    max_cps_slider: slider::State,

    ver_jitter_intensity_slider: slider::State,
    hor_jitter_intensity_slider: slider::State,

    //toggle_button: button::State // work in progress, not supported yet.
}

impl UserInterface for ClickerGUI {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, msg: Message, _window: &mut Window) {
        match msg {
            Message::MinClicksChanged(data) => {
                self.data.min_cps = data as u64;
            },

            Message::MaxClicksChanged(data) => {
                self.data.max_cps = data as u64;
            },

            Message::VerticalJitterIntensityChanged(data) => {
                self.data.jitter_intensity_vertical = data as i32;
            },

            Message::HorizontalJitterIntensityChanged(data) => {
                self.data.jitter_intensity_horizontal = data as i32;
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        let mut clicks = Column::new().max_width(250).spacing(20);
        let mut jitter = Column::new().max_width(250).spacing(20);

        jitter = jitter.push(
            slider_with_label(
                "horizontal jitter intensity: ",
                &mut self.hor_jitter_intensity_slider,
                0.0..=25.0,
                self.data.jitter_intensity_horizontal as f32,
                &ToString::to_string(&self.data.jitter_intensity_horizontal),
                move |data| {
                    Message::HorizontalJitterIntensityChanged(data)
                }   
            )
        );

        jitter = jitter.push(
            slider_with_label(
                "vertical jitter intensity: ",
                &mut self.ver_jitter_intensity_slider,
                0.0..=25.0,
                self.data.jitter_intensity_vertical as f32,
                &ToString::to_string(&self.data.jitter_intensity_vertical),
                move |data| {
                    Message::VerticalJitterIntensityChanged(data)
                }   
            )
        );

        clicks = clicks.push(
            slider_with_label(
                "minimum cps: ",
                &mut self.min_cps_slider,
                5.0..=25.0,
                self.data.min_cps as f32,
                &ToString::to_string(&self.data.min_cps),
                move |data| {
                    Message::MinClicksChanged(data)
                }   
            )
        );

        clicks = clicks.push(
            slider_with_label(
                "maximum cps: ",
                &mut self.max_cps_slider,
                5.0..=25.0,
                self.data.max_cps as f32,
                &ToString::to_string(&self.data.max_cps),
                move |data| {
                    Message::MaxClicksChanged(data)
                }   
            )
        );
        
        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .padding(20)
            .align_items(Align::Stretch)
            .justify_content(Justify::SpaceBetween)
            .push(jitter)
            .push(clicks)
            .into()
    }
}

impl Game for ClickerGUI {
    type Input = Mouse;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<ClickerGUI> {
        Task::succeed(|| ClickerGUI {
            data: ClickerData::new(),

            min_cps_slider: slider::State::new(),
            max_cps_slider: slider::State::new(),

            hor_jitter_intensity_slider: slider::State::new(),
            ver_jitter_intensity_slider: slider::State::new()
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
        
        self.data.handle_listeners();
    }
}

fn slider_with_label<'a>(label: &str, state: &'a mut slider::State, range: RangeInclusive<f32>, value: f32, format: &str, on_change: fn(f32) -> Message,) -> Element<'a, Message> {
    Column::new()
        .spacing(10)
        .push(Text::new(label))
        .push(
            Row::new()
                .spacing(10)
                .push(Slider::new(state, range, value, on_change))
                .push(
                    Text::new(format)
                        .width(150)
                        .height(50)
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
        )
        .into()
}