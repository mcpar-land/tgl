pub struct DialogFile(pub Vec<Dialog>);

impl DialogFile {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn from_list(list: &[Dialog]) -> Self {
		Self(list.to_vec())
	}

	pub fn first_key(&self) -> &str {
		self.0[0].key.as_str()
	}
}

#[derive(Clone, Debug)]
pub struct Dialog {
	pub key: String,
	pub value: Vec<DialogComponent>,
	pub goto: Vec<(Goto, String)>,
}

impl Dialog {
	pub fn new(
		key: &str,
		value: &[DialogComponent],
		goto: &[(Goto, String)],
	) -> Self {
		Self {
			key: key.to_string(),
			value: value.to_vec(),
			goto: goto.to_vec(),
		}
	}
}

#[derive(Clone, Debug)]
pub enum DialogComponent {
	Text(String),
	Effect(String),
	Pause(usize),
	Speed(f32),
}

impl DialogComponent {
	pub fn text(text: &str) -> Self {
		Self::Text(text.to_string())
	}
}

#[derive(Clone, Debug)]
pub enum Goto {
	Dialog(String),
	End,
}

impl Goto {
	pub fn dialog(id: &str) -> Self {
		Self::Dialog(id.to_string())
	}
}

fn scaffolding_dialog() -> DialogFile {
	let mut f = DialogFile::from_list(&[
		Dialog::new(
			"scaff-1",
			&[DialogComponent::text("This is a text of the first dialog.")],
			&[(Goto::dialog("scaff-2"), "Go to second one".to_string())],
		),
		Dialog::new(
			"scaff-2",
			&[DialogComponent::text("This is the second dialog. It is very long and involved. It goes on for a very long time. As you can see, this is the longest dialog.")],
			&[
				(Goto::dialog("scaff-3"), "Go to third one".to_string()),
				(Goto::dialog("scaff-4"), "Go to fourth one".to_string()),
				(Goto::dialog("scaff-5"), "Go to fifth one".to_string()),
			],
		),
		Dialog::new(
			"scaff-3",
			&[DialogComponent::text("This is the third one.")],
			&[(Goto::End, "Finish up".to_string())],
		),
		Dialog::new(
			"scaff-4",
			&[DialogComponent::text("This is the fourth one.")],
			&[(Goto::End, "Finish up".to_string())],
		),
		Dialog::new(
			"scaff-5",
			&[DialogComponent::text("This is the fifth one.")],
			&[(Goto::End, "Finish up".to_string())],
		),
	]);
	todo!();
}
