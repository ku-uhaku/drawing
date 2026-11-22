# Geometrical Shapes
 
A small Rust library for drawing basic geometric shapes onto an image. Shapes don't know anything about the image itself: they just send pixels (`x`, `y`, `color`) to anything that implements the `Displayable` trait. This makes it easy to draw on a `raster::Image`, a test canvas, or your own buffer.
 
## Features
 
- Five shapes: `Point`, `Line`, `Circle`, `Triangle`, `Rectangle`
- Random shape generation (`random`) for `Point`, `Line` and `Circle`
- Two small traits (`Displayable`, `Drawable`) that keep drawing and storage separate
- Unit tests that use a fake canvas, so no image file is needed
## Requirements
 
- Rust (stable, 2021 edition or newer)
- Dependencies in `Cargo.toml`:
```toml
[dependencies]
raster = "0.2"
rand = "0.9"   # `rand::random_range` needs 0.9+
```
 
## Project layout
 
```
.
├── Cargo.toml
├── README.md
└── src
    └── lib.rs      # traits, shapes, and tests
```
 
## Traits
 
```rust
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
 
pub trait Drawable {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}
```
 
| Trait         | Who implements it                | Purpose                                              |
|---------------|----------------------------------|------------------------------------------------------|
| `Displayable` | The image / canvas you draw on   | Sets a single pixel at `(x, y)` to a color           |
| `Drawable`    | Every shape                      | `draw` sends the shape's pixels to a `Displayable`; `color` returns the shape's color |
 
`display` receives raw `i32` coordinates, which can be negative or past the image edge (for example a circle partly outside the image). Your `Displayable` implementation should ignore pixels that don't fit.
 
## Shapes
 
| Shape       | Constructor                                   | `random`? | How it's drawn                          |
|-------------|-----------------------------------------------|-----------|------------------------------------------|
| `Point`     | `Point::new(x, y)`                            | Yes       | One white pixel                          |
| `Line`      | `Line::new(&p1, &p2)`                         | Yes       | Linear interpolation between endpoints   |
| `Circle`    | `Circle::new(&center, radius)`                | Yes       | Midpoint circle algorithm (outline)      |
| `Triangle`  | `Triangle::new(&p1, &p2, &p3)`                | No        | Three lines (outline)                    |
| `Rectangle` | `Rectangle::new(&corner1, &corner2)`          | No        | Four lines (outline)                     |
 
### Details
 
- **Colors:** shapes made with `new` are white. `Line::random` and `Circle::random` also pick a random color. `Point` is always white.
- **`random(width, height)`:** points (and line endpoints, and circle centers) are placed inside `0..width` and `0..height`. A random circle's radius is `0..255` and is *not* limited by the image size, so it can extend past the edges.
- **`Line`:** works with `i64` internally, so very long lines don't overflow. A line whose endpoints are identical draws a single pixel.
- **`Circle`:** a negative radius is treated as `0` (a single pixel).
- **`Rectangle`:** takes two opposite corners in any order and draws the four sides.
- **`Triangle`:** draws the three sides in the shape's color.
## Example
 
Implement `Displayable` for `raster::Image`, then draw some shapes and save the result:
 
```rust
use geometrical_shapes::{Circle, Displayable, Drawable, Line, Point, Rectangle, Triangle};
use raster::{Color, Image};
 
struct Canvas(Image);
 
impl Displayable for Canvas {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        // Ignore pixels outside the image
        if x >= 0 && x < self.0.width && y >= 0 && y < self.0.height {
            self.0.set_pixel(x, y, color).unwrap();
        }
    }
}
 
fn main() {
    let mut canvas = Canvas(Image::blank(1000, 1000));
 
    let a = Point::new(100, 100);
    let b = Point::new(900, 700);
    let c = Point::new(100, 700);
 
    Line::new(&a, &b).draw(&mut canvas);
    Rectangle::new(&a, &b).draw(&mut canvas);
    Triangle::new(&a, &b, &c).draw(&mut canvas);
    Circle::new(&Point::new(500, 500), 200).draw(&mut canvas);
 
    // A few random shapes
    for _ in 0..20 {
        Line::random(1000, 1000).draw(&mut canvas);
        Circle::random(1000, 1000).draw(&mut canvas);
        Point::random(1000, 1000).draw(&mut canvas);
    }
 
    raster::save(&canvas.0, "image.png").unwrap();
}
```
 
## Writing your own shape
 
Implement `Drawable` and send pixels with `image.display(...)`:
 
```rust
struct Cross { center: Point, size: i32 }
 
impl Drawable for Cross {
    fn draw(&self, image: &mut dyn Displayable) {
        let (x, y) = (self.center.x, self.center.y);
        for i in -self.size..=self.size {
            image.display(x + i, y, self.color());
            image.display(x, y + i, self.color());
        }
    }
 
    fn color(&self) -> Color {
        Color::white()
    }
}
```
 
## Tests
 
```
cargo test
```
 
The tests use a small fake `Canvas` that records every pixel it receives, so they don't touch any image file. They check that:
 
- random points, line endpoints and circle centers stay inside the image (including 1-pixel-wide images)
- a rectangle draws the same outline no matter which corners are passed first
- a line with identical endpoints draws exactly one pixel
## Limitations
 
- Shapes are outlines only. There is no fill.
- `Triangle` and `Rectangle` have no `random` constructor yet (the code is there, commented out).
- `Circle::draw` uses `i32` arithmetic, so a center near `i32::MAX` / `i32::MIN` can overflow.
 
