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
            color: ColorGenerator::Uniform(RGB(0x66, 0xFF, 0x30)),
        }
    };
    let cube1 = Shape {
        primitive : Primitive::TriangleList,
        verts: vec!(
            Vector3(150.0,  50.1, 0.5),
            Vector3(150.0,  50.1, 0.8),
            Vector3(250.0,  50.1, 0.5),
            Vector3(150.0,  50.1, 0.8),
            Vector3(250.0,  50.1, 0.5),
            Vector3(250.0,  50.1, 0.8),

            Vector3(250.0,  50.1, 0.5),
            Vector3(250.0, 150.1, 0.5),
            Vector3(250.0,  50.1, 0.8),
            Vector3(250.0, 150.1, 0.5),
            Vector3(250.0,  50.1, 0.8),
            Vector3(250.0, 150.1, 0.8),

            Vector3(150.0, 150.1, 0.5),
            Vector3(150.0, 150.1, 0.8),
            Vector3(250.0, 150.1, 0.5),
            Vector3(150.0, 150.1, 0.8),
            Vector3(250.0, 150.1, 0.5),
            Vector3(250.0, 150.1, 0.8),

            Vector3(150.0,  50.1, 0.5),
            Vector3(150.0, 150.1, 0.5),
            Vector3(150.0,  50.1, 0.8),
            Vector3(150.0, 150.1, 0.5),
            Vector3(150.0,  50.1, 0.8),
            Vector3(150.0, 150.1, 0.8),

            Vector3(150.0,  50.1, 0.5),
            Vector3(150.0, 150.1, 0.5),
            Vector3(250.0,  50.1, 0.5),
            Vector3(150.0, 150.1, 0.5),
            Vector3(250.0,  50.1, 0.5),
            Vector3(250.0, 150.1, 0.5),

            Vector3(150.0,  50.1, 0.8),
            Vector3(150.0, 150.1, 0.8),
            Vector3(250.0,  50.1, 0.8),
            Vector3(150.0, 150.1, 0.8),
            Vector3(250.0,  50.1, 0.8),
            Vector3(250.0, 150.1, 0.8),
        ),
        material: Material {
            color: ColorGenerator::Linear(
                RGB(0x66, 0x32, 0x00),
                RGB(0x34, 0x00, 0x22),
                RGB(0xFF, 0x66, 0x00))
        }
    };
    let cube2 = Shape {
        primitive : Primitive::TriangleList,
        verts: vec!(
            Vector3(400.0,  50.1, 0.5),
            Vector3(400.0,  50.1, 0.8),
            Vector3(550.0,  50.1, 0.5),
            Vector3(400.0,  50.1, 0.8),
            Vector3(550.0,  50.1, 0.5),
            Vector3(550.0,  50.1, 0.8),

            Vector3(550.0,  50.1, 0.5),
            Vector3(550.0, 400.1, 0.5),
            Vector3(550.0,  50.1, 0.8),
            Vector3(550.0, 400.1, 0.5),
            Vector3(550.0,  50.1, 0.8),
            Vector3(550.0, 400.1, 0.8),

            Vector3(400.0, 400.1, 0.5),
            Vector3(400.0, 400.1, 0.8),
            Vector3(550.0, 400.1, 0.5),
            Vector3(400.0, 400.1, 0.8),
            Vector3(550.0, 400.1, 0.5),
            Vector3(550.0, 400.1, 0.8),

            Vector3(400.0,  50.1, 0.5),
            Vector3(400.0, 400.1, 0.5),
            Vector3(400.0,  50.1, 0.8),
            Vector3(400.0, 400.1, 0.5),
            Vector3(400.0,  50.1, 0.8),
            Vector3(400.0, 400.1, 0.8),

            Vector3(400.0,  50.1, 0.5),
            Vector3(400.0, 400.1, 0.5),
            Vector3(550.0,  50.1, 0.5),
            Vector3(400.0, 400.1, 0.5),
            Vector3(550.0,  50.1, 0.5),
            Vector3(550.0, 400.1, 0.5),

            Vector3(400.0,  50.1, 0.8),
            Vector3(400.0, 400.1, 0.8),
            Vector3(550.0,  50.1, 0.8),
            Vector3(400.0, 400.1, 0.8),
            Vector3(550.0,  50.1, 0.8),
            Vector3(550.0, 400.1, 0.8),
        ),
        material: Material {
            color: ColorGenerator::Linear(
                RGB(0x00, 0xFF, 0x00),
                RGB(0xFF, 0x00, 0xFF),
                RGB(0xFF, 0x00, 0xFF))
        }
    };
    World {
        shapes: vec!(floor, cube1, cube2),
        lighting: vec!(light)
    }
}
