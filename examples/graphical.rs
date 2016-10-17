extern crate hexagon;
extern crate lux;

use lux::prelude::*;
use lux::graphics::{ColorVertex, PrimitiveType, PrimitiveCanvas};
use hexagon::*;
use hexagon::screen::ScreenSpace;

fn draw_hex(frame: &mut Frame, screen: &ScreenSpace, hex: &HexPosition, color: [f32; 4], size: f32) {
    fn get_color_vertex((x, y): (f32, f32), color: [f32; 4]) -> ColorVertex {
        ColorVertex {
            pos: [x, y],
            color: color,
        }
    }

    let hex_positions = screen.points_on_tile_custom_size(&hex, size);

    frame.draw_colored(PrimitiveType::TriangleFan, &[
        get_color_vertex(hex_positions[0], color),
        get_color_vertex(hex_positions[1], color),
        get_color_vertex(hex_positions[2], color),
        get_color_vertex(hex_positions[3], color),
        get_color_vertex(hex_positions[4], color),
        get_color_vertex(hex_positions[5], color),
    ], None, None);
}

fn main() {
    let mut window = Window::new_with_defaults().unwrap();

    let hex = HexPosition::origin();
    let screenspace = ScreenSpace {
        size: 100.0,
        origin: (300.0, 300.0),
    };

    while window.is_open() {
        let mut frame = window.cleared_frame((0.0, 0.0, 0.0));
        let (x, y) = window.mouse_pos();

        draw_hex(&mut frame, &screenspace, &hex, [1.0, 1.0, 1.0, 1.0], 100.0);

        /*
        for neighbor in hex.ring_with_radius(2) {
            draw_hex(&mut frame, &screenspace, &neighbor, [1.0, 0.0, 0.0, 1.0], 100.0);
        }*/

        let (r1, r2) = hex.bidirectional_ray(4);
        for pos in r1.take(10).skip(1) {
            draw_hex(&mut frame, &screenspace, &pos, [1.0, 0.0, 0.0, 1.0], 100.0);
        }
        for pos in r2.take(10).skip(1) {
            draw_hex(&mut frame, &screenspace, &pos, [1.0, 0.0, 0.0, 1.0], 100.0);
        }

        let near_cursor = &screenspace.nearest_hex(x, y);
        draw_hex(&mut frame, &screenspace, near_cursor, [0.0, 1.0, 0.0, 1.0], 100.0);
        draw_hex(&mut frame, &screenspace, near_cursor, [0.0, 0.0, 0.0, 1.0], 90.0);
    }
}
