#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        // Gravity constant
        let g = 9.8;

        // Increment time by 1 second
        self.time += 1.0;

        // New velocity
        let new_velocity_y = self.init_velocity.y - g * self.time;

        // New position
        let new_position_y = self.init_position.y + self.init_velocity.y * self.time - 0.5 * g * self.time * self.time;
        let new_position_x = self.init_position.x + self.init_velocity.x * self.time;

        // If the object is below the floor
        if new_position_y < 0.0 {
            return None;
        }

        // Update current velocity and position
        self.actual_velocity = Object {
            x: self.init_velocity.x,
            y: (new_velocity_y*10.0).round() / 10.0,
        };
        self.actual_position = Object {
            x: (new_position_x*10.0).round() / 10.0,
            y: (new_position_y*10.0).round() / 10.0,
        };

        Some(self.clone())
    }
}
