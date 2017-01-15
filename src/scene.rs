use geometry::*;

pub fn scene() -> World {
    let light = Vector3(2.0, 2.0, 2.0);
    let floor = Shape {
        primitive : Primitive::TriangleStrip,
        verts: vec!(
            Vector3(100.0, 50.0, 3.0),
            Vector3(100.0, 50.0, 0.1),
            Vector3(600.0, 50.0, 3.0),
            Vector3(600.0, 50.0, 0.1),
        ),
        material: Material {
            color: ColorGenerator::Uniform(RGB(0x33, 0x33, 0x33)),
        }
    };
    World {
        shapes: vec!(floor),
        lighting: vec!(light)
    }
}
