use fltk::dialog;
use fltk::app::get_mouse;
pub(crate) use dialog::color_chooser_with_default;
pub(crate) use dialog::ColorMode;

pub(crate) fn input_default(txt: &str, deflt: &str) -> Option<String> {
    let (x,y) = get_mouse();
    dialog::input(x, y, txt, deflt)
}

pub(crate) fn alert_default(txt: &str) {
    let (x,y) = get_mouse();
    dialog::alert(x, y, txt)
}

pub(crate) fn message_default(txt: &str) {
    let (x,y) = get_mouse();
    dialog::message(x, y, txt)
}

pub(crate) fn choice_default(txt: &str, b0: &str, b1: &str) -> i32 {
    let (x,y) = get_mouse();
    dialog::choice(x, y, txt, b0, b1, "")
}
