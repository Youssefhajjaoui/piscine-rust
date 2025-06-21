pub mod areas_volumes;
use areas_volumes::{
    circle_area, cone_volume, cube_volume, parallelepiped_volume, rectangle_area, sphere_volume,
    square_area, triangle_area, triangular_pyramid_volume,
};

pub type GeometricalShapes = areas_volumes::GeometricalShapes;
pub type GeometricalVolumes = areas_volumes::GeometricalVolumes;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area = match kind {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Circle => circle_area(a) as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Triangle => triangle_area(a, b) as usize,
    };
    return (x * y) > area * times;
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let volume = match kind {
        GeometricalVolumes::Cube => cube_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b) as usize,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c),
        GeometricalVolumes::Sphere => sphere_volume(a) as usize,
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b) as usize,
    };
    return (x * y * z) > volume * times;
}
