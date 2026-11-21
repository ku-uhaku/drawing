use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable  {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}


#[derive(Debug)]
pub struct Point{
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }

    pub fn random(width:i32, height: i32) -> Self {
        return Self { x: rand::random_range(0..width), y: rand::random_range(0..height) };
    }
}


#[derive(Debug)]
pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        return Self { p1, p2 };
    }  

    pub fn random(width: i32, height: i32) -> (Self, Color) {
        return (
            Self { p1: Point::random(width, height), p2: Point::random(width, height)}, 
            Color::rgb(rand::random_range(0..255), rand::random_range(0..255),rand::random_range(0..255)),
        )
    } 
}