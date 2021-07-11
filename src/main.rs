mod clicker;
mod support;

use clicker::ClickerData;
use imgui::*;

fn main() {
    let system = support::init("RClicker", &[768f64, 400f64]);
    let mut clicker = ClickerData::new();

    system.main_loop(move |_, ui| {
        create_ui(ui, &mut clicker);
        clicker.handle_listeners()
    })
}

/// Formats a boolean to a String
fn format_bool(input: bool) -> String {
    match input {
        true => "Yes",
        false => "No",
    }.to_owned()
}

/// Creates the UI
fn create_ui(ui: &mut Ui, clicker: &mut ClickerData) {
	if clicker.min_cps > clicker.max_cps {
		clicker.min_cps = clicker.max_cps;
	}
    Window::new(im_str!("RClicker"))
        .size([768.0, 400.0], Condition::Appearing)
        .position([0.0, 0.0], Condition::Appearing)
        .title_bar(false)
        .resizable(false)
        .movable(false)
		.collapsible(false)
        .build(ui, || {
            Slider::new(im_str!("Min CPS"))
                .range(5..=22)
                .build(&ui, &mut clicker.min_cps);

            Slider::new(im_str!("Max CPS"))
                .range(8..=25)
                .build(&ui, &mut clicker.max_cps);

            ui.separator();

            Slider::new(im_str!("Horizontal Jitter"))
                .range(0..=20)
                .build(&ui, &mut clicker.jitter_intensity_horizontal);

            Slider::new(im_str!("Vertical Jitter"))
                .range(0..=20)
                .build(&ui, &mut clicker.jitter_intensity_vertical);

            Slider::new(im_str!("Debounce-Time (experimental)"))
                .range(0..=20)
                .build(&ui, &mut clicker.debounce_time);

			ComboBox::new(im_str!("Button"))
				.build_simple_string(&ui, &mut clicker.selected_button, &[im_str!("Left"), im_str!("Right")]);

            ui.separator();

            ui.text(format!("Clicker enabled: {}", format_bool(clicker.enabled)));
            ui.text(format!(
                "Horizontal Jitter enabled: {}",
                format_bool(clicker.jitter_intensity_horizontal != 0)
            ));
            ui.text(format!(
                "Vertical Jitter enabled: {}",
                format_bool(clicker.jitter_intensity_vertical != 0)
            ));
			ui.text(format!(
				"Debounce-Time enabled: {}",
				format_bool(clicker.debounce_time != 0)
			))
        });
}
