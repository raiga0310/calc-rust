fn plus<T: std::ops::Add<Output = T> + Copy>(arg1: T, arg2: T) -> T {
	arg1 + arg2
}

fn main() {
	println!("Hello, plus!! 0.55 + 0.73 is {}", plus(0.55, 0.73));
	println!("And 5 + 7 is {}", plus(5, 7));
}
