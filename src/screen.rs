use super::HexPosition;

pub struct ScreenSpace {
    pub size: f32,
    pub origin: (f32, f32),
}

impl ScreenSpace {
    pub fn pixel_coords_for_center(&self, pos: &HexPosition) -> (f32, f32) {
        let x = self.size * (3.0f32).sqrt() * (pos.x as f32 + pos.z as f32 / 2.0);
        let y = self.size * (3.0 / 2.0) * pos.z as f32;
        (x + self.origin.0, y + self.origin.1)
    }

    pub fn nearest_hex(&self, x: f32, y: f32) -> HexPosition {
        fn round(x: f32, y: f32, z: f32) -> HexPosition {
            let mut rx = x.round();
            let mut ry = y.round();
            let mut rz = z.round();

            let x_diff = (rx - x).abs();
            let y_diff = (ry - y).abs();
            let z_diff = (rz - z).abs();

            if x_diff > y_diff && x_diff > z_diff {
                rx = -ry-rz
            } else if y_diff > z_diff {
                ry = -rx-rz
            } else {
                rz = -rx-ry
            }

            HexPosition {
                x: rx as i32,
                y: ry as i32,
                z: rz as i32,
            }
        }
        let x = x - self.origin.0;
        let y = y - self.origin.0;

        let q = (x * 3.0f32.sqrt() / 3.0 - y / 3.0) / self.size;
        let r = y * (2.0 / 3.0) / self.size;

        round(q, 0.0 - q - r, r)
    }

    pub fn points_on_tile_custom_size(&self, position: &HexPosition, size: f32) -> [(f32, f32); 6] {
        fn hex_corner(center: (f32, f32), size: f32, i: u32) -> (f32, f32) {
            let i = i as f32;
            let angle_deg = 60.0 * i + 30.0;
            let angle_rad = ::std::f32::consts::PI / 180.0 * angle_deg;
            return (center.0 + size * angle_rad.cos(),
                    center.1 + size * angle_rad.sin())
        }

        let center = self.pixel_coords_for_center(position);

        [
            hex_corner(center, size, 0),
            hex_corner(center, size, 1),
            hex_corner(center, size, 2),
            hex_corner(center, size, 3),
            hex_corner(center, size, 4),
            hex_corner(center, size, 5),
        ]
    }

    pub fn points_on_tile(&self, position: &HexPosition) -> [(f32, f32); 6] {
        self.points_on_tile_custom_size(position, self.size)
    }

    pub fn height_of_tile(&self) -> f32 {
        self.size * 2.0
    }

    pub fn width_of_tile(&self) -> f32 {
        ((3.0f32).sqrt() / 2.0) * self.height_of_tile()
    }
}
