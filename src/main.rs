mod rgb;
mod screen;
mod primitives;
use screen::Screen;
use rgb::RGB;

fn main() {
    console_error_panic_hook::set_once();

    let screen = Screen::init();

    for x in 0..=screen.width {
        for y in 0..=screen.height {
            screen.render(x, y, RGB {red: x, green: y, blue: (x + y) / 2});
        }
    }
}
