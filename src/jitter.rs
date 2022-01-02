pub type JitterFn = fn((usize, usize), f64) -> (f32, f32);

pub fn jitter_sin((x, y): (usize, usize), time: f64) -> (f32, f32) {
	(
		0.0,
		((time as f32 + x as f32 + y as f32 * 20.0) * 20.0).sin(),
	)
}

pub fn jitter_noise((x, y): (usize, usize), time: f64) -> (f32, f32) {
	let x = (time * 100.0).sin() as f32 * 10000.0;
	let y = (time.sin() * 100.0) as f32 * 56347.0;
	let offset = 2.0;
	((x - x.floor()) * offset, (y - y.floor()) * offset)
}
