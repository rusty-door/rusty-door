use geometry::*;

pub fn scene() -> World {
    let light1 = Vector3(0.0, 200.0, 1.0);
    let light2 = Vector3(500.0, 200.0, 0.0);
    let floor = Shape {
        primitive : Primitive::TriangleStrip,
        verts: vec!(
            Vector3(100.0, 50.0, 3.0),
            Vector3(100.0, 50.0, 0.1),
            Vector3(600.0, 50.0, 3.0),
            Vector3(600.0, 50.0, 0.1),
        ),
        material: Material {
            color: ColorGenerator::SphereTexture(
                Vector3(350.0, 50.0, 1.45),
                Texture2d {
                    width: 4,
                    height: 4,
                    values: vec!(
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        RGB(0x70, 0x40, 0x00),
                        RGB(0xB0, 0x40, 0x00),
                        )})
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
            color: ColorGenerator::Uniform(RGB(0x15, 0x00, 0x30))
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
            color: ColorGenerator::Uniform(RGB(0x00, 0x40, 0x30))

        }
    };
    let cube3 = Shape {
        primitive : Primitive::TriangleList,
        verts: vec!(
            Vector3(450.0,  50.1, 0.40),
            Vector3(450.0,  50.1, 0.41),
            Vector3(460.0,  50.1, 0.40),
            Vector3(450.0,  50.1, 0.41),
            Vector3(460.0,  50.1, 0.40),
            Vector3(460.0,  50.1, 0.41),

            Vector3(460.0,  50.1, 0.40),
            Vector3(460.0,  70.1, 0.40),
            Vector3(460.0,  50.1, 0.41),
            Vector3(460.0,  70.1, 0.40),
            Vector3(460.0,  50.1, 0.41),
            Vector3(460.0,  70.1, 0.41),

            Vector3(450.0,  70.1, 0.40),
            Vector3(450.0,  70.1, 0.41),
            Vector3(460.0,  70.1, 0.40),
            Vector3(450.0,  70.1, 0.41),
            Vector3(460.0,  70.1, 0.40),
            Vector3(460.0,  70.1, 0.41),

            Vector3(450.0,  50.1, 0.40),
            Vector3(450.0,  70.1, 0.40),
            Vector3(450.0,  50.1, 0.41),
            Vector3(450.0,  70.1, 0.40),
            Vector3(450.0,  50.1, 0.41),
            Vector3(450.0,  70.1, 0.41),

            Vector3(450.0,  50.1, 0.40),
            Vector3(450.0,  70.1, 0.40),
            Vector3(460.0,  50.1, 0.40),
            Vector3(450.0,  70.1, 0.40),
            Vector3(460.0,  50.1, 0.40),
            Vector3(460.0,  70.1, 0.40),

            Vector3(450.0,  50.1, 0.41),
            Vector3(450.0,  70.1, 0.41),
            Vector3(460.0,  50.1, 0.41),
            Vector3(450.0,  70.1, 0.41),
            Vector3(460.0,  50.1, 0.41),
            Vector3(460.0,  70.1, 0.41),
        ),
        material: Material {
            color: ColorGenerator::Uniform(RGB(0x78, 0x00, 0x30))
        }
    };
    World {
        shapes: vec!(floor, cube1, cube2, cube3),
        lighting: vec!(light1, light2)
    }
}
