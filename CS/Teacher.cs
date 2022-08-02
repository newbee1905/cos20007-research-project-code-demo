namespace StudentManagementSystem;

public class Teacher : StudentPresident
{
    public Teacher(string name, short age) : base(name, age)
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
