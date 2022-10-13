use rapier3d::prelude::*;
fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Make the ground
    let ground_collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
    collider_set.insert(ground_collider);

    // Make the ball
    // Steps, build a rigid body, then a collider, then insert them into the sets
    // Dynamic rigidbody is meant to move under simulation.

    let ball_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 5.0, 0.0])
        .build();
    let ball_collider = ColliderBuilder::ball(0.5).restitution(1.1).build();
    let ball_handle = rigid_body_set.insert(ball_rigid_body);
    collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);
    // Set up the scene and physics constants.
    let gravity = vector![0.0, -9.81, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new(); // Figure out what this does
    let mut multibody_joint_set = MultibodyJointSet::new(); // Figure out what this does
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = (); // Figure out what this does
    let event_handler = (); // Figure out what this does
                            // Run the game loop
    for _ in 0..1000 {
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
        let ball_body = rigid_body_set.get(ball_handle).unwrap();
        println!(
            "Ball position: {:?}",
            ball_body.position().translation.vector
        );
    }
}
