use kiss3d;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::Translation3;
use kiss3d::window::Window;
use rapier3d::prelude::*;
fn main() {
    // The first iteration of this game is gonna use a "super class" to hold and manipulate all the game objects.
    // Initialize the game.
    let mut window = Window::new("Mini Project: Bowling Game");
    let mut sphere = window.add_sphere(1.0);
    sphere.set_color(1.0, 0.0, 0.0); // Make it red
    window.set_light(Light::StickToCamera);
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    // Make the ball
    let ball_collider = ColliderBuilder::ball(0.9).restitution(0.75).build();
    let ball_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![2.0, 0.0, 0.0]) // No clue what the default is, hopefully it's a zero vector
        .build();
    // set up the physics
    let gravity = vector![0.0, -9.81, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new(); // Figure out what this does
    let mut multibody_joint_set = MultibodyJointSet::new(); // Figure out what this does
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = (); // Figure out what this does
    let event_handler = (); // Figure out what this does

    // End physics
    // Set up pipelines and colliders
    let ball_handle = rigid_body_set.insert(ball_rigid_body);

    collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);
    let mut physics_pipeline = PhysicsPipeline::new();
    // make a rigidbody and collider set
    make_lane(&mut window, &mut rigid_body_set, &mut collider_set);
    // Render the scene
    while window.render() {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(kiss3d::event::Key::Space, Action::Press, _) => {
                    println!("Spacebar pressed");
                    physics_pipeline.step(
                        &gravity,
                        &integration_parameters,
                        &mut island_manager,
                        &mut broad_phase,
                        &mut narrow_phase,
                        &mut rigid_body_set,
                        &mut collider_set,
                        &mut impulse_joint_set,
                        &mut multibody_joint_set,
                        &mut ccd_solver,
                        &physics_hooks,
                        &event_handler,
                    );
                    let ball_rigid_body = rigid_body_set.get(ball_handle).unwrap();
                    let ball_rigid_translation = ball_rigid_body.position();
                    let thing = Translation3::new(
                        ball_rigid_translation.translation.x,
                        ball_rigid_translation.translation.y,
                        ball_rigid_translation.translation.z,
                    );
                    sphere.prepend_to_local_translation(&thing);
                }
                _ => {}
            }
        }
    }
}

pub fn make_lane(window: &mut Window, bodies: &mut RigidBodySet, colliders: &mut ColliderSet) {
    let lane_rigid_body = RigidBodyBuilder::fixed().build(); // Make the lane as a fixed rigid body and collider.
    let lane_handle = bodies.insert(lane_rigid_body);
    let side_size = 0.4;
    // Make two walls with a floor

    // let left_wall_collider = ColliderBuilder::cuboid(side_size * 10.0, side_size, side_size * 5.0);
    // let right_wall_collider = ColliderBuilder::cuboid(side_size * 10.0, side_size, side_size * 5.0);
    // let floor_colliders = ColliderBuilder::cuboid(side_size * 10.0, side_size, side_size * 5.0);
    // colliders.insert_with_parent(left_wall_collider, lane_handle, bodies);
    // colliders.insert_with_parent(right_wall_collider, lane_handle, bodies);
    // colliders.insert_with_parent(floor_colliders, lane_handle, bodies);
    let lane_shape = vec![
        (
            Isometry::identity(),
            SharedShape::cuboid(side_size * 10.0, side_size, side_size * 5.0),
        ),
        (
            Isometry::translation(side_size * 10.0, side_size * 10.0, 0.0),
            SharedShape::cuboid(side_size * 10.0, side_size, side_size * 5.0),
        ),
        (
            Isometry::translation(-side_size * 10.0, 0.0, 0.0),
            SharedShape::cuboid(side_size, side_size, side_size * 10.0),
        ),
    ];
    let lane_collider = ColliderBuilder::compound(lane_shape).build();

    colliders.insert_with_parent(lane_collider, lane_handle, bodies);

    let mut lane_group = window.add_group();
    // let mut lane = window.add_cube(1.0, 0.1, 1.0); // Make the lane as a scene node.
    lane_group.add_cube(side_size * 10.0, side_size, side_size * 5.0);
    lane_group
        .add_cube(side_size * 10.0, side_size, side_size * 5.0)
        .append_translation(&Translation3::new(side_size * 10.0, side_size * 10.0, 0.0));
    lane_group
        .add_cube(side_size, side_size, side_size * 10.0)
        .append_translation(&Translation3::new(-side_size * 10.0, 0.0, 0.0));

    lane_group.set_color(0.0, 0.0, 1.0);
}
