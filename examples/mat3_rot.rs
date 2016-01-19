extern crate halcyon;
extern crate kiss3d;
extern crate nalgebra;

use nalgebra::Pnt3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use halcyon::mat3_rot::*;

fn main() {
    let mut window = Window::new("Matrix3D rotation");
    window.set_light(Light::StickToCamera);

    //let mut cube   = window.add_cube(1.0, 1.0, 1.0);
    //cube.set_color(1.0, 0.0, 0.0);

    let rot = rot(0.3, 0.3, 0.3);

    while window.render() {
        draw_line(&mut window, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        draw_line(&mut window, [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        draw_line(&mut window, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);

        // TODO cleanup
        draw_line(&mut window, [0.0, 0.0, 0.0], mul_mat_vec(rot, [1.0, 0.0, 0.0]), [1.0, 0.0, 0.0]);
        draw_line(&mut window, [0.0, 0.0, 0.0], mul_mat_vec(rot, [0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]);
        draw_line(&mut window, [0.0, 0.0, 0.0], mul_mat_vec(rot, [0.0, 0.0, 1.0]), [1.0, 0.0, 0.0]);
    }
}

fn draw_line(window: &mut Window, from: [f32; 3], to: [f32; 3], color: [f32; 3]) {
    window.draw_line(
        &Pnt3::new(from[0], from[1], from[2]),
        &Pnt3::new(to[0], to[1], to[2]),
        &Pnt3::new(color[0], color[1], color[2]));
}
