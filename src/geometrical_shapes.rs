use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable  {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}

