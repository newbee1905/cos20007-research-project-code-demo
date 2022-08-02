use crate::student::{Student, StudentPresident, StudentVicePresident};
use crate::teacher::Teacher;
use std::collections::HashMap;

pub type Grades = HashMap<String, i8>;
pub type AttendanceChecker = HashMap<String, bool>;
pub type AttendanceList = HashMap<String, AttendanceChecker>;

pub struct Lecture {
	name: String,
	address: String,
	time_start: String,
	time_stop: String,
	homeroom_teacher: Teacher,
	students: Vec<Student>,
	student_president: Option<StudentPresident>,
	student_vicepresident: Option<StudentVicePresident>,
	grades: Grades,
	attendance_list: AttendanceList
}

impl Lecture {
	pub fn new(name: &str, address: &str, time_start: &str, time_stop: &str, teacher: &Teacher) -> Self {
		Self {
			name: name.into(),
			address: address.into(),
			time_start: time_start.into(),
			time_stop: time_stop.into(),
			homeroom_teacher: teacher.clone(),
			student_president: None,
			student_vicepresident: None,
			students: Vec::new(),
			grades: Grades::new(),
			attendance_list: AttendanceList::new(),
		}
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn get_address(&self) -> &str {
		&self.address
	}

	pub fn get_time_start(&self) -> &str {
		&self.time_start
	}

	pub fn get_time_stop(&self) -> &str {
		&self.time_stop
	}

	pub fn get_students(&self) -> &Vec<Student> {
		&self.students
	}

	pub fn get_student_vicepresident(&self) -> &Option<StudentVicePresident> {
		&self.student_vicepresident
	}

	pub fn get_student_president(&self) -> &Option<StudentPresident> {
		&self.student_president
	}

	pub fn get_grades(&mut self) -> &mut HashMap<String, i8> {
		&mut self.grades
	}

	pub fn get_attendance_list(&mut self) -> &mut HashMap<String, HashMap<String, bool>> {
		&mut self.attendance_list
	}
}
