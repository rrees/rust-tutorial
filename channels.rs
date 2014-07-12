
fn main() {
	let numbers = [1, 2, 3];

	let (port, channel) = Chan::new();

	channel.send(numbers);

	do spawn {
		let numbers = port.recv();
		println!("{:d}", numbers[0]);
	}
}