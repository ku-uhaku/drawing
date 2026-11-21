use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self {
            x: rand::random_range(0..width),
            y: rand::random_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut dyn Displayable) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0)
    }
}

#[derive(Debug)]
pub struct Line {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Line {
    pub fn new(p1: Point, p2: Point, color: Color) -> Self {
        Self { p1, p2, color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
            color: Color::rgb(
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
            ),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut dyn Displayable) {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;

        let steps = dx.abs().max(dy.abs());

        if steps == 0 {
            image.display(self.p1.x, self.p1.y, self.color());
            return;
        }

        for i in 0..=steps {
            let x = self.p1.x + dx * i / steps;
            let y = self.p1.y + dy * i / steps;

            image.display(x, y, self.color());
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}


