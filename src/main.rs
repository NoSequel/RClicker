mod clicker;
mod support;

use clicker::ClickerData;
use imgui::*;

fn main() {
    let system = support::init(file!(), &[768f64, 400f64]);
    let mut clicker = ClickerData::new();

    system.main_loop(move |_, ui| {
        create_ui(ui, &mut clicker);
        clicker.handle_listeners()
    })
}

fn create_ui(ui: &mut Ui, clicker: &mut ClickerData) {
    Window::new(im_str!("r clicker"))
        .size([768.0, 400.0], Condition::Appearing)
        .position([0.0, 0.0], Condition::Appearing)
        .resizable(false)
        .movable(false)
        .build(ui, || {
            Slider::new(im_str!("min cps"))
                .range(8..=25)
                .build(&ui, &mut clicker.min_cps);

            Slider::new(im_str!("max cps"))
                .range(5..=22)
                .build(&ui, &mut clicker.max_cps);

            ui.separator();

            Slider::new(im_str!("hor jitter intensity"))
                .range(0..=20)
                .build(&ui, &mut clicker.jitter_intensity_horizontal);

            Slider::new(im_str!("ver jitter intensity"))
                .range(0..=20)
                .build(&ui, &mut clicker.jitter_intensity_vertical);

            Slider::new(im_str!("debounce time (experimental)"))
                .range(0..=20)
                .build(&ui, &mut clicker.debounce_time);

            ui.separator();

            ui.text(format!("clicker enabled: {}", clicker.enabled));
            ui.text(format!(
                "horizontal jitter enabled: {}",
                clicker.jitter_intensity_horizontal != 0
            ));
            ui.text(format!(
                "veritcal jitter enabled: {}",
                clicker.jitter_intensity_vertical != 0
            ));
        });
}
