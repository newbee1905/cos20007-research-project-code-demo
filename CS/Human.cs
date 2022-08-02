namespace StudentManagementSystem;

public abstract class Human
{
    private string _name;
    private short _age;

    public Human(string name, short age)
    {
        _name = name;
        _age = age;
    }

    public string Name => _name;
    public short Age => _age;
}
