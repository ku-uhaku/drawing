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
         Color::rgb(
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
            )
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

#[cfg(test)]
mod tests {
    use super::*;

    struct Counter(i32);

    impl Displayable for Counter {
        fn display(&mut self, x: i32, y: i32, color: Color) {
            self.0 += 1;
        }
    }

    #[test]
    fn random_point_and_line() {
        for _ in 0..1000 {
            let p = Point::random(100, 50);

            assert!(p.x >= 0 && p.x < 100);
            assert!(p.y >= 0 && p.y < 50);

            let l = Line::random(100, 50);

            for p in [&l.p1, &l.p2] {
                assert!(p.x >= 0 && p.x < 100);
                assert!(p.y >= 0 && p.y < 50);
            }
        }
    }

    #[test]
    fn edge_case_point_and_line() {
        let line = Line::new(
            Point::new(1, 1),
            Point::new(1, 1),
            Color::rgb(255, 0, 0),
        );

        let mut counter = Counter(0);

        line.draw(&mut counter);

        assert_eq!(counter.0, 1);
    }
}