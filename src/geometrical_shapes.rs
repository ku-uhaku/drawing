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

    #[derive(Default)]
    struct Canvas(Vec<(i32, i32)>);

    impl Displayable for Canvas {
        fn display(&mut self, x: i32, y: i32, _color: Color) {
            self.0.push((x, y));
        }
    }

    #[test]
    fn random_points_and_lines_stay_inside_image() {
        for (width, height) in [(1, 1), (1, 12), (12, 1), (100, 50)] {
            for _ in 0..100 {
                let point = Point::random(width, height);
                let line = Line::random(width, height);
                for p in [&point, &line.p1, &line.p2] {
                    assert!((0..width).contains(&p.x));
                    assert!((0..height).contains(&p.y));
                }
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
            canvas.0.sort_unstable();
            canvas.0.dedup();
            assert_eq!(canvas.0, expected);
        }
    }

    #[test]
    fn identical_line_endpoints_draw_one_pixel() {
        let line = Line::new(&Point::new(1, 1), &Point::new(1, 1), Color::white());
        let mut canvas = Canvas::default();
        line.draw(&mut canvas);
        assert_eq!(canvas.0, vec![(1, 1)]);
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

// adding cube

pub struct Cube {
    p1: Point,
    size: i32,
    color: Color,
}

impl Cube {
    pub fn new(p1: &Point, size: i32) -> Self {
        Self {
            p1: *p1,
            size,
            color: Color::rgb(
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
            ),
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut dyn Displayable) {
        let x = self.p1.x;
        let y = self.p1.y;
        let size = self.size;
        let offset = size / 4;
        let front = [
            Point::new(x, y),
            Point::new(x + size, y),
            Point::new(x + size, y + size),
            Point::new(x, y + size),
        ];
        let back = front.map(|p| Point::new(p.x + offset, p.y + offset));

        Rectangle {
            p1: front[0],
            p2: front[2],
            color: self.color(),
        }
        .draw(image);
        Rectangle {
            p1: back[0],
            p2: back[2],
            color: self.color(),
        }
        .draw(image);

        for i in 0..4 {
            Line::new(&front[i], &back[i], self.color()).draw(image);
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}
