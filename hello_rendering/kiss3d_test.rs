use kiss3d;
use kiss3d::light::Light;
use kiss3d::window::Window;
use rapier3d::na::{UnitQuaternion, Vector3};
// pub extern crate nalgebra as na;
// use rapier3d::prelude::*;
fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut cube = window.add_cube(1.0, 1.0, 1.0);
    cube.set_color(1.0, 0.0, 0.0);
    window.set_light(Light::StickToCamera);
    let axis = Vector3::y_axis();
    let rotation = UnitQuaternion::from_axis_angle(&axis, 0.02);
    while window.render() {
        cube.prepend_to_local_rotation(&rotation);
    }
}
