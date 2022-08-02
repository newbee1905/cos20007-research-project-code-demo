namespace StudentManagementSystem;

public class Student : Human
{
    public Student(string name, short age) : base(name, age)
    { }

    public byte GetGrade(Lecture lecture)
        => lecture.Grades[this.Name];
}

public class StudentPresident : Student
{
    public StudentPresident(string name, short age) : base(name, age)
    { }

    public void CheckAttendance(Lecture lecture, string studentName, string date, bool check)
    {
        lecture.AttendanceList.TryGetValue(studentName, out Dictionary<string, bool>? studentAttendanceList);
        if (studentAttendanceList is null)
        {
            studentAttendanceList = new();
            lecture.AttendanceList.Add(studentName, studentAttendanceList);
        }
        bool exist = studentAttendanceList.TryGetValue(date, out bool curCheck);
        if (exist)
            curCheck = check;
        else
            studentAttendanceList.Add(date, check);
    }
}


public class StudentVicePresident : Student
{
    public StudentVicePresident(string name, short age) : base(name, age)
    { }

    public void Grade(Lecture lecture, string studentName, byte grade)
    {
        bool exist = lecture.Grades.TryGetValue(studentName, out byte curGrade);
        if (exist)
            curGrade = grade;
        else
            lecture.Grades.Add(studentName, grade);
    }
}
