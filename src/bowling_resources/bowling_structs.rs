#![allow(dead_code)]
use kiss3d;
use rapier3d::prelude::*;
pub struct BowlingBall {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub handle: Option<RigidBodyHandle>,
}
impl BowlingBall {
    pub fn new() -> BowlingBall {
        let ball_rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vector::default()) // No clue what the default is, hopefully it's a zero vector
            .build();
        let ball_collider = ColliderBuilder::ball(0.9).restitution(0.75).build();
        let ball_handle = None;
        // This is a placeholder for the handle that will be assigned when the ball is inserted into the rigidbody set.
        // Once it's inserted, then we can retrieve a handle.
        // If there's a better way to do this, I'm all ears.
        BowlingBall {
            rigid_body: ball_rigid_body,
            collider: ball_collider,
            handle: ball_handle,
        }
    }
}
pub struct BowlingPhysics {
    pub gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub hooks: Box<dyn PhysicsHooks>,
    pub events: Box<dyn EventHandler>,
}
impl BowlingPhysics {
    // Yes, I'm hard coding these because there's no way I'm managing lifetimes for all of these.
    fn new() -> BowlingPhysics {
        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();
        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new(); // Figure out what this does
        let multibody_joint_set = MultibodyJointSet::new(); // Figure out what this does
        let ccd_solver = CCDSolver::new();
        let physics_hooks = (); // Figure out what this does
        let event_handler = (); // Figure out what this does
        BowlingPhysics {
            gravity,
            integration_parameters,
            island_manager,
            broad_phase,
            narrow_phase,
            bodies: rigid_body_set,
            colliders: collider_set,
            impulse_joints: impulse_joint_set,
            multibody_joints: multibody_joint_set,
            ccd_solver,
            hooks: Box::new(physics_hooks),
            events: Box::new(event_handler),
        }
    }
}
pub struct BowlingGame {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub window: kiss3d::window::Window,
    physics: BowlingPhysics,
    pipeline: PhysicsPipeline,
}
// implement bowling game
impl BowlingGame {
    pub fn new() -> Self {
        let mut window = kiss3d::window::Window::new("Mini Project: Bowling Game");
        let mut sphere = window.add_sphere(1.0);
        sphere.set_color(1.0, 0.0, 0.0); // Make it red
        window.set_light(kiss3d::light::Light::StickToCamera);
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut bowling_ball = BowlingBall::new();
        bowling_ball.handle = Some(rigid_body_set.insert(bowling_ball.rigid_body));
        collider_set.insert_with_parent(
            bowling_ball.collider,
            bowling_ball
                .handle
                .expect("Bowling ball handle is None when creating game."),
            &mut rigid_body_set,
        );
        let physics = BowlingPhysics::new();
        let physics_pipeline = PhysicsPipeline::new();
        // Relinquinsh ownership of the window and various physics items to the BowlingGame struct.
        BowlingGame {
            rigid_body_set,
            collider_set,
            window,
            physics,
            pipeline: physics_pipeline,
        }
    }
    // Make a step forward. Update handles the physics steps and maybe translating the
    pub fn update(&mut self) {
        // Update the scene
    }
    pub fn render(&mut self) {
        // Render the scene
        while self.window.render() {
            self.update();
        }
    }
}

pub struct BowlingLane {
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

pub struct BowlingPin {
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

pub struct BowlingPinSet {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
}
