use crate::human::Human;
use crate::lecture::{AttendanceChecker, Lecture};

use crate::{human_getters, human_new};

pub trait IGrader {
	fn grade(lecture: &mut Lecture, student_name: &str, grade: i8) {
		let grades = lecture.get_grades();

		*grades.entry(student_name.into()).or_insert(grade) = grade;
	}
}

pub trait IAttendanceChecker {
	fn check_attendance(lecture: &mut Lecture, student_name: &str, date: &str, check: bool) {
		let attendance_list = lecture.get_attendance_list();

		let student_attendance_list = attendance_list
			.entry(student_name.into())
			.or_insert(AttendanceChecker::new());

		*student_attendance_list.entry(date.into()).or_insert(check) = check;
	}
}

#[derive(Debug, Clone)]
pub struct Teacher {
	name: String,
	age: i8,
}

impl Teacher {
	human_new!();
}

impl Human for Teacher {
	human_getters!();
}

impl IGrader for Teacher {}
impl IAttendanceChecker for Teacher {}
