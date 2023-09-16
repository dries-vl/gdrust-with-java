use godot::prelude::*;
use godot::engine::{Sprite2D, Sprite2DVirtual};

/// Registers the class in Godot; requires engine restart
/// Inherits Sprite2D; if omitted it inherits RefCounted
#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    /// Needed to access parent class instance because rust has no inheritance
    /// Can omit if no need for base class within self
    #[base]
    sprite: Base<Sprite2D>
}

/// Needed to expose this also to gdextension;
/// Every godot engine class has a {ClassName} Virtual trait
#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        self.sprite.rotate((self.angular_speed * delta) as f32);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}
