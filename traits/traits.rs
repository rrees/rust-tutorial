
struct Circle {
	radius: f64,
}

impl Area for Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

trait Area {
	fn area(&self) -> f64;
}

fn main() {
	let c = Circle {
		radius: 5.0,
	};

	println!("Area is {}", c.area());
}