mod areas_volumes; 
pub use crate::areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let total_area = (x * y) as f64;

    let shape_area = match kind {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };

    total_area >= shape_area * times as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let total_volume = (x * y * z) as f64;
    let shape_volume = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Cone => cone_volume(a, b),
    };

    total_volume >= shape_volume * times as f64
}
