use imgui::*;
use imgui_knobs::*;

use imgui_baseview::{HiDpiMode, ImguiWindow, RenderSettings, Settings};

use crate::filter_parameters::FilterParameters;
use crate::parameter::Parameter;
use crate::utils::*;
// for now just using the original parameter struct
// use super::FilterParameters;
use vst::editor::Editor;

use baseview::{Size, WindowOpenOptions, WindowScalePolicy};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::sync::Arc;

const WINDOW_WIDTH: usize = 512;
const WINDOW_HEIGHT: usize = 256;
const WINDOW_WIDTH_F: f32 = WINDOW_WIDTH as f32;
const WINDOW_HEIGHT_F: f32 = WINDOW_HEIGHT as f32;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const BG_COLOR: [f32; 4] = [0.21 * 1.4, 0.11 * 1.7, 0.25 * 1.4, 1.0];
// const BG_COLOR_TRANSP: [f32; 4] = [0.21 * 1.4, 0.11 * 1.7, 0.25 * 1.4, 0.0];
// const GREEN: [f32; 4] = [0.23, 0.68, 0.23, 1.0];
// const RED: [f32; 4] = [0.98, 0.02, 0.22, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.58, 0.0, 1.0];
const ORANGE_HOVERED: [f32; 4] = [1.0, 0.68, 0.1, 1.0];
// const WAVEFORM_LINES: [f32; 4] = [1.0, 1.0, 1.0, 0.2];
const TEXT: [f32; 4] = [1.0, 1.0, 1.0, 0.75];
// const DB_LINES: [f32; 4] = [1.0, 1.0, 1.0, 0.15];

pub fn draw_knob(knob: &Knob, wiper_color: &ColorSet, track_color: &ColorSet) {
    knob.draw_arc(
        0.8,
        0.20,
        knob.angle_min,
        knob.angle_max,
        track_color,
        16,
        2,
    );
    if knob.t > 0.01 {
        knob.draw_arc(0.8, 0.21, knob.angle_min, knob.angle, wiper_color, 16, 2);
    }
}
/// Meant for general knobs
pub fn make_knob(
    ui: &Ui,
    parameter: &Parameter<AtomicF32>,
    // parameter_index: i32,
    wiper_color: &ColorSet,
    track_color: &ColorSet,
    title_fix: f32,
) {
    let width = ui.text_line_height() * 4.75;
    let w = ui.push_item_width(width);
    // let title = parameter.get_name();
    let title = parameter.get_name();
    let knob_id = &ImString::new(format!("##{}_KNOB_CONTORL_", title));
    knob_title(ui, &ImString::new(title.clone().to_uppercase()), width);
    let cursor = ui.cursor_pos();
    ui.set_cursor_pos([cursor[0], cursor[1] + 5.0]);
    let mut val = parameter.get();
    let knob = Knob::new(
        ui,
        knob_id,
        &mut val,
        parameter.min,
        parameter.max,
        parameter.default,
        width * 0.5,
        true,
    );
    let cursor = ui.cursor_pos();
    ui.set_cursor_pos([cursor[0] + title_fix, cursor[1] - 10.0]);
    knob_title(ui, &ImString::new(parameter.get_display()), width);

    if knob.value_changed {
        // TODO: FIXME: Something needs to happen here to change the parameter in the small window
        parameter.set(*knob.p_value);
        knob_title(ui, &ImString::new("value change happened"), width);
    }

    w.pop(ui);
    draw_knob(&knob, wiper_color, track_color);
}
/// Meant for knobs that go through discrete steps. Nowhere close to done.
// pub fn make_steppy_knob(
//     ui: &Ui,
//     parameter: &Parameter<AtomicUsize>,
//     // parameter_index: i32,
//     wiper_color: &ColorSet,
//     track_color: &ColorSet,
//     title_fix: f32,
// ) {
//     let width = ui.text_line_height() * 4.75;
//     let w = ui.push_item_width(width);
//     // let title = parameter.get_name();
//     let title = parameter.get_name();
//     let knob_id = &ImString::new(format!("##{}_KNOB_CONTORL_", title));
//     knob_title(ui, &ImString::new(title.clone().to_uppercase()), width);
//     let cursor = ui.cursor_pos();
//     ui.set_cursor_pos([cursor[0], cursor[1] + 5.0]);
//     let mut val = parameter.get();
//     let knob = Knob::new(
//         ui,
//         knob_id,
//         &mut val,
//         parameter.min,
//         parameter.max,
//         parameter.default,
//         width * 0.5,
//         true,
//     );
//     let cursor = ui.cursor_pos();
//     ui.set_cursor_pos([cursor[0] + title_fix, cursor[1] - 10.0]);
//     knob_title(ui, &ImString::new(parameter.get_display()), width);

//     if knob.value_changed {
//         parameter.set(*knob.p_value)
//     }

//     w.pop(ui);
//     draw_knob(&knob, wiper_color, track_color);
// }
pub struct EditorState {
    pub params: Arc<FilterParameters>,
    // pub sample_rate: Arc<AtomicFloat>,
    // pub time: Arc<AtomicFloat>,
}
pub struct SVFPluginEditor {
    pub is_open: bool,
    pub state: Arc<EditorState>,
}
fn move_cursor(ui: &Ui, x: f32, y: f32) {
    let cursor = ui.cursor_pos();
    ui.set_cursor_pos([cursor[0] + x, cursor[1] + y])
}

fn _floating_text(ui: &Ui, text: &str) {
    ui.get_window_draw_list()
        .add_text(ui.cursor_pos(), ui.style_color(StyleColor::Text), text)
}
impl Editor for SVFPluginEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        //::log::info!("self.running {}", self.running);
        if self.is_open {
            return false;
        }

        self.is_open = true;

        let settings = Settings {
            window: WindowOpenOptions {
                title: String::from("imgui-baseview demo window"),
                size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
            },
            clear_color: (0.0, 0.0, 0.0),
            hidpi_mode: HiDpiMode::Default,
            render_settings: RenderSettings::default(),
        };

        ImguiWindow::open_parented(
            &VstParent(parent),
            settings,
            self.state.clone(),
            |ctx: &mut Context, _state: &mut Arc<EditorState>| {
                ctx.fonts().add_font(&[FontSource::TtfData {
                    data: include_bytes!("../OpenSans-Semibold.ttf"),
                    size_pixels: 20.0,
                    config: None,
                }]);
            },
            |_run: &mut bool, ui: &Ui, state: &mut Arc<EditorState>| {
                // {
                //     let mut editor_only = state.editor_only.lock().unwrap();
                //     editor_only.sample_data.consume();
                // }
                //ui.show_demo_window(run);
                let w = Window::new(im_str!("Example 1: Basic sliders"))
                    .size([WINDOW_WIDTH_F, WINDOW_HEIGHT_F], Condition::Appearing)
                    .position([0.0, 0.0], Condition::Appearing)
                    .draw_background(false)
                    .no_decoration()
                    .movable(false);
                w.build(&ui, || {
                    let text_style_color = ui.push_style_color(StyleColor::Text, TEXT);
                    let graph_v_center = 225.0 + 25.0;
                    // {
                    //     let draw_list = ui.get_window_draw_list();
                    //     draw_list.add_rect_filled_multicolor(
                    //         [0.0, 0.0],
                    //         [WINDOW_WIDTH_F, 200.0],
                    //         BLACK,
                    //         BLACK,
                    //         BG_COLOR,
                    //         BG_COLOR,
                    //     );
                    //     draw_list
                    //         .add_rect([0.0, 200.0], [WINDOW_WIDTH_F, WINDOW_HEIGHT_F], BG_COLOR)
                    //         .filled(true)
                    //         .build();
                    //     draw_list
                    //         .add_rect(
                    //             [0.0, graph_v_center - 92.0],
                    //             [WINDOW_WIDTH_F, graph_v_center + 92.0],
                    //             [0.0, 0.0, 0.0, 0.65],
                    //         )
                    //         .filled(true)
                    //         .build();
                    // }
                    ui.set_cursor_pos([0.0, 25.0]);

                    let highlight = ColorSet::new(ORANGE, ORANGE_HOVERED, ORANGE_HOVERED);

                    let params = &state.params;

                    let _line_height = ui.text_line_height();
                    let n_columns = 5;
                    let lowlight = ColorSet::from(BLACK);
                    ui.columns(n_columns, im_str!("cols"), false);
                    let width = WINDOW_WIDTH_F / n_columns as f32 - 0.25;
                    for i in 1..n_columns {
                        ui.set_column_width(i, width);
                    }
                    ui.set_column_width(0, width * 0.5);

                    ui.next_column();
                    make_knob(ui, &params.cutoff, &highlight, &lowlight, 0.0);
                    move_cursor(ui, 0.0, -113.0);

                    ui.next_column();

                    make_knob(ui, &params.res, &highlight, &lowlight, 0.0);
                    ui.next_column();

                    make_knob(ui, &params.drive, &highlight, &lowlight, 0.0);
                    ui.next_column();

                    // make_steppy_knob(ui, &params.mode, &highlight, &lowlight, 0.0);
                    ui.next_column();

                    ui.columns(1, im_str!("nocols"), false);

                    move_cursor(ui, 0.0, 84.0);

                    text_style_color.pop(ui);
                });
            },
        );

        true
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
    }
}
struct VstParent(*mut ::std::ffi::c_void);

#[cfg(target_os = "macos")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::macos::MacOSHandle;

        RawWindowHandle::MacOS(MacOSHandle {
            ns_view: self.0 as *mut ::std::ffi::c_void,
            ..MacOSHandle::empty()
        })
    }
}

#[cfg(target_os = "windows")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::windows::WindowsHandle;

        RawWindowHandle::Windows(WindowsHandle {
            hwnd: self.0,
            ..WindowsHandle::empty()
        })
    }
}

#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::unix::XcbHandle;

        RawWindowHandle::Xcb(XcbHandle {
            window: self.0 as u32,
            ..XcbHandle::empty()
        })
    }
}
