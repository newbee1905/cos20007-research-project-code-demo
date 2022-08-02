use crate::human::Human;
use crate::lecture::Lecture;
use crate::teacher::{IGrader, IAttendanceChecker};

use crate::{human_getters, human_new};

pub trait IStudent {
	fn get_grade(&self, lecture: &mut Lecture) -> i8;
}

macro_rules! student_function {
	{} => {
		fn get_grade(&self, lecture: &mut Lecture) -> i8 {
			match lecture.get_grades().get(&self.name) {
				Some(&grade) => grade,
				_ => -1
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct Student {
	name: String,
	age: i8,
}

impl Student {
	human_new!();
}

impl Human for Student {
	human_getters!();
}

impl IStudent for Student {
	student_function!();
}

#[derive(Debug, Clone)]
pub struct StudentPresident {
	name: String,
	age: i8,
}

impl StudentPresident {
	human_new!();
}

impl Human for StudentPresident {
	human_getters!();
}

impl IStudent for StudentPresident {
	student_function!();
}

impl IAttendanceChecker for StudentPresident {}

#[derive(Debug, Clone)]
pub struct StudentVicePresident {
	name: String,
	age: i8,
}

impl StudentVicePresident {
	human_new!();
}

impl Human for StudentVicePresident {
	human_getters!();
}

impl IStudent for StudentVicePresident {
	student_function!();
}

impl IGrader for StudentVicePresident {}
