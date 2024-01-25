use enum_variants::{enum_variant, EnumVairant};

struct PointData {
    x: i32,
    y: i32,
}

#[derive(EnumVairant)]
enum Point {
    #[value(PointData { x: -1, y: 0 })]
    LEFT,
    #[value(PointData { x: 1, y: 0 })]
    RIGHT,
}

enum_variant! {
    enum Point2 {
        LEFT2 = PointData { x: -1, y: 0 },
        RIGHT2 = PointData { x: 1, y: 0 },
    }
}

fn main() {
    println!("Hello, world!");
}
