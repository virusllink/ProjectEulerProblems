using System;
using System.Numerics;

class Program
{
    static void Main()
    {
        string[] fileText = File.ReadAllText("names.txt").Split("\",\"");
        Array.Sort(fileText);
        long answer = 0;

        for (int i = fileText.Length - 1; i >= 0; i--)
        {
            char[] chars = fileText[i].ToCharArray();
            int placeholder = 0;
            for (int j = 0; j < chars.Length; j++)
            {
                placeholder += chars[j] - 64;
            }
            placeholder *= (i + 1);
            answer += placeholder;
        }

        Console.WriteLine(answer);

        Console.ReadKey();
    }
}