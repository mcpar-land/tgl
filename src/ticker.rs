// use crate::dialog::dialog::DialogFile;

pub struct Ticker {
	// pub file: DialogFile,
	pub current_id: String,
	pub tick_position: usize,
	pub wait_time: f32,
	pub y: usize,
}

impl Ticker {
	// pub fn new(file: DialogFile) -> Self {
	// 	let current_id = file.first_key().to_string();
	// 	Self {
	// 		file,
	// 		current_id,
	// 		tick_position: 0,
	// 		wait_time: 0.0,
	// 		y: 0,
	// 	}
	// }

	// pub fn reset(&mut self, file: DialogFile) {
	// 	self.file = file;
	// 	self.current_id = self.file.first_key().to_string();
	// 	self.tick_position = 0;
	// 	self.wait_time = 0.0;
	// 	self.y = 0;
	// }
}
