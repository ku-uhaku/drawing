use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self::new(rand::random_range(0..width), rand::random_range(0..height))
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
    pub fn new(p1: &Point, p2: &Point, color: Color) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            color,
        }
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
        let dx = i64::from(self.p2.x) - i64::from(self.p1.x);
        let dy = i64::from(self.p2.y) - i64::from(self.p1.y);

        let steps = dx.abs().max(dy.abs());

        if steps == 0 {
            image.display(self.p1.x, self.p1.y, self.color());
            return;
        }

        for i in 0..=steps {
            let x = i64::from(self.p1.x) + dx * i / steps;
            let y = i64::from(self.p1.y) + dy * i / steps;

            image.display(x as i32, y as i32, self.color());
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
        fn display(&mut self, _x: i32, _y: i32, _color: Color) {
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
        let line = Line::new(&Point::new(1, 1), &Point::new(1, 1), Color::white());

        let mut counter = Counter(0);

        line.draw(&mut counter);

        assert_eq!(counter.0, 1);
    }

    #[derive(Default)]
    struct Canvas(Vec<(i32, i32, (u8, u8, u8, u8))>);

    fn rgba(color: Color) -> (u8, u8, u8, u8) {
        (color.r, color.g, color.b, color.a)
    }

    impl Displayable for Canvas {
        fn display(&mut self, x: i32, y: i32, color: Color) {
            self.0.push((x, y, rgba(color)));
        }
    }

    #[test]
    fn random_drawings_stay_inside_image() {
        for (width, height) in [(1, 1), (1, 12), (12, 1), (2, 2), (100, 50)] {
            for _ in 0..100 {
                let mut canvas = Canvas::default();
                Point::random(width, height).draw(&mut canvas);
                Line::random(width, height).draw(&mut canvas);
                Circle::random(width, height).draw(&mut canvas);
                assert!(!canvas.0.is_empty());
                assert!(
                    canvas
                        .0
                        .iter()
                        .all(|&(x, y, _)| (0..width).contains(&x) && (0..height).contains(&y))
                );
            }
        }
    }

    #[test]
    fn rectangle_handles_reversed_corners() {
        let mut expected = Vec::new();
        for x in 2..=5 {
            expected.extend([(x, 3), (x, 7)]);
        }
        for y in 4..7 {
            expected.extend([(2, y), (5, y)]);
        }
        expected.sort_unstable();
        for ((x1, y1), (x2, y2)) in [
            ((2, 3), (5, 7)),
            ((5, 7), (2, 3)),
            ((2, 7), (5, 3)),
            ((5, 3), (2, 7)),
        ] {
            let mut canvas = Canvas::default();
            Rectangle::new(&Point::new(x1, y1), &Point::new(x2, y2)).draw(&mut canvas);
            let mut actual: Vec<_> = canvas.0.iter().map(|&(x, y, _)| (x, y)).collect();
            actual.sort_unstable();
            actual.dedup();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn shapes_keep_their_color_when_drawn() {
        let p = Point::new(3, 4);
        let line = Line::new(&p, &Point::new(7, 8), Color::white());
        let rectangle = Rectangle::new(&p, &Point::new(7, 8));
        let triangle = Triangle::new(&p, &Point::new(7, 4), &Point::new(5, 8));
        let circle = Circle::new(&p, 2);
        for shape in [&line as &dyn Drawable, &rectangle, &triangle, &circle] {
            let color = rgba(shape.color());
            let mut canvas = Canvas::default();
            shape.draw(&mut canvas);
            shape.draw(&mut canvas);
            assert!(!canvas.0.is_empty());
            assert_eq!(rgba(shape.color()), color);
            assert!(canvas.0.iter().all(|pixel| pixel.2 == color));
        }
    }

    #[test]
    fn zero_radius_circle_draws_only_center() {
        let mut canvas = Canvas::default();
        Circle::new(&Point::new(8, 9), 0).draw(&mut canvas);
        assert!(!canvas.0.is_empty());
        assert!(canvas.0.iter().all(|&(x, y, _)| (x, y) == (8, 9)));
    }
}

// Triangle and Rectangle

#[derive(Debug)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    color: Color,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            p3: *p3,
            color: Color::rgb(
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
            ),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut dyn Displayable) {
        Line::new(&self.p1, &self.p2, self.color.clone()).draw(image);
        Line::new(&self.p2, &self.p3, self.color.clone()).draw(image);
        Line::new(&self.p3, &self.p1, self.color.clone()).draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            color: Color::rgb(
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
            ),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let a = &self.p1;
        let b = &Point::new(self.p2.x, self.p1.y);
        let c = &self.p2;
        let d = &Point::new(self.p1.x, self.p2.y);
        Line::new(a, b, self.color.clone()).draw(image);
        Line::new(b, c, self.color.clone()).draw(image);
        Line::new(c, d, self.color.clone()).draw(image);
        Line::new(d, a, self.color.clone()).draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
