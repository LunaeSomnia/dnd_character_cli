use colored::{ColoredString, Colorize};

pub enum Color {
    Red,
    RedBG,
    Yellow,
    YellowBG,
    Blue,
    BlueBG,
    Cyan,
    CyanBG,
    Green,
    GreenBG,
    Magenta,
    MagentaBG,
    Grey,
    GreyBG
}

pub trait ToColor {
    fn to_color(&self, background: bool) -> Color;
}

pub fn colorize_string(str: &str, color: Color) -> ColoredString {
    let colored = match color {
        Color::Red => str.red(),
        Color::RedBG => str.on_red(),
        Color::Yellow => str.bright_yellow(),
        Color::YellowBG => str.on_bright_yellow(),
        Color::Blue => str.blue(),
        Color::BlueBG => str.on_blue(),
        Color::Cyan => str.cyan(),
        Color::CyanBG => str.on_cyan(),
        Color::Green => str.green(),
        Color::GreenBG => str.on_green(),
        Color::Magenta => str.magenta(),
        Color::MagentaBG => str.on_magenta(),
        Color::Grey => str.bright_black(),
        Color::GreyBG => str.on_bright_black()
    };
    colored
}

pub fn colorize_string_with_style(str: &str, color: Color, bold: bool, italic: bool) -> ColoredString {
    let colorized = colorize_string(str, color);
    let colorized = match bold {
        true => colorized.bold(),
        false => colorized
    };
    let colorized = match italic {
        true => colorized.italic(),
        false => colorized
    };
    colorized
}