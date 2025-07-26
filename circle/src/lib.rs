#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(x:f64,y:f64,r :f64) -> Circle {
        Circle { center: Point(x, y), radius: r }
    }
    pub fn diameter(self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    pub fn intersect(self, c: Circle) ->bool {
        self.center.distance(c.center) <= self.radius+c.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64,pub f64);

impl Point {
    pub fn distance(self: Point,to:Point)-> f64 {
        ((self.0 - to.0).powi(2)+ (self.1 - to.1).powi(2)).sqrt()
    }

}