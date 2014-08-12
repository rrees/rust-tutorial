
struct Circle {
	radius: f64,
}

struct Square {
	side: f64,
}

trait Area {
	fn area(&self) -> f64;
}

impl Area for Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

impl Area for Square {
	fn area(&self) -> f64 {
		self.side * self.side
	}
}

fn main() {
	let c = Circle {
		radius: 5.0,
	};

	println!("Area of a circle is {}", c.area());

	let s = Square {
		side: 4.0,
	};

	println!("Area of a square is {}", s.area());
}