use super::rgb::RGB;

pub trait Colour {
    fn at_brightness_as_rgb(&self, brightness: usize) -> &RGB;
}
