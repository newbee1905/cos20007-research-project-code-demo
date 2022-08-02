namespace StudentManagementSystem;

using System.Collections.Generic;

public class Lecture
{
    private string _name;
    private string _address;
    private string _timeStart;
    private string _timeStop;
    private List<Student> _students;
    private Teacher _homeroomTeacher;
    private StudentPresident? _studentPresident;
    private StudentVicePresident? _studentVicePresident;
    private Dictionary<String, byte> _grades;
    private Dictionary<String, Dictionary<String, bool>> _attendanceList;

    public Lecture(string name, string address, string timeStart, string timeStop, Teacher teacher)
    {
        _name = name;
        _address = address;
        _timeStart = timeStart;
        _timeStop = timeStop;
        _students = new();
        _homeroomTeacher = teacher;
				_grades = new();
				_attendanceList = new();
    }

    public string Name => _name;
    public string Address => _address;
    public string Start => _timeStart;
    public string Stop => _timeStop;
    public List<Student> Students => _students;
    public Teacher HomeroomTeacher { get => _homeroomTeacher; set => _homeroomTeacher = value; }
    public StudentPresident? StudentPresident { get => _studentPresident; set => _studentPresident = value; }
    public StudentVicePresident? StudentVicePresident { get => _studentVicePresident; set => _studentVicePresident = value; }
    public Dictionary<String, byte> Grades => _grades;
    public Dictionary<String, Dictionary<String, bool>> AttendanceList => _attendanceList;
}
