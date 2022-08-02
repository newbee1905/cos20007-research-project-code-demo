pub trait Human {
	fn get_name(&self) -> &str;
	fn get_age(&self) -> i8;
}

#[macro_export]
macro_rules! human_new {
	{} => {
		pub fn new(name: &str, age: i8) -> Self {
			Self { name: name.into(), age }
		}
	}
}

#[macro_export]
macro_rules! human_getters {
	{} => {
		fn get_name(&self) -> &str {
			&self.name
		}
		fn get_age(&self) -> i8 {
			self.age
		}
	}
}
