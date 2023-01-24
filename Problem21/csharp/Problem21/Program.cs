class Program
{
    static void Main()
    {
        int sumOfAmicableNumbers = 0;
        for (int i = 2; i < 10000; i++)
        {
            //Get divisors of i
            int[] properDivisorsNum1 = GetDivisors(i);
            //Get the sum of divisors of i
            int sumOfProperDivisorsNum1 = 0;
            foreach(int item in properDivisorsNum1)
            {
                sumOfProperDivisorsNum1 += item;
            }

            //Get divisors of sumOfProperDivisorsNum1
            int[] properDivisorsNum2 = GetDivisors(sumOfProperDivisorsNum1);
            //Get the sum of divisors of sumOfProperDivisorsNum1
            int sumOfProperDivisorsNum2 = 0;
            foreach (int item in properDivisorsNum2)
            {
                sumOfProperDivisorsNum2 += item;
            }

            //Check if sumOfProperDivisorsNum1 and sumOfProperDivisorsNum2 are amicable numbers
            if (i == sumOfProperDivisorsNum2 && sumOfProperDivisorsNum1 != sumOfProperDivisorsNum2)
            {
                sumOfAmicableNumbers += sumOfProperDivisorsNum1;
            }
        }
        Console.WriteLine(sumOfAmicableNumbers);
    }

    public static int[] GetDivisors(int n)
    {
        List<int> divisors = new List<int>();
        for (int i = 1; i <= Math.Sqrt(n); i++)
        {
            if (n % i == 0)
            {
                divisors.Add(i);
                if (i != n / i)
                {
                    divisors.Add(n / i);
                }
            }
        }
        divisors.Sort();
        if(divisors[divisors.Count-1] == n)
        {
            divisors.RemoveAt(divisors.Count-1);
        }
        return divisors.ToArray();
    }
}