// use bowling_resources::bowling_structs::BowlingGame;
use kiss3d;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::Translation3;
use kiss3d::window::Window;
use rapier3d::prelude::*;
mod bowling_resources;
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
