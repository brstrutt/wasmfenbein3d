use super::rgbv::RGBV;

pub trait TexelProvider {
    fn get_texel(&self, x: isize, y: isize) -> &RGBV;

    fn width(&self) -> usize;
    fn width_f64(&self) -> &f64;

    fn height(&self) -> usize;
    fn height_f64(&self) -> &f64;
}
