using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace Client
{
    class Program
    {
        [DllImport("api", EntryPoint = "sum_of_even")]
        private static extern int SumOfEven(int[] numbers, UIntPtr len);

        public static int SumOfEven(int[] numbers)
        {
            return SumOfEven(numbers, (UIntPtr)(numbers == null ? 0 : numbers.Length));
        }

        static void TestApi()
        {
            void AssertSumOfEven(int[] numbers, int expectedResult)
            {
                var result = SumOfEven(numbers);

                Debug.Assert(expectedResult == result);
            }

            AssertSumOfEven(null, -1);
            AssertSumOfEven(new int[] { }, 0);
            AssertSumOfEven(new[] { 0 }, 0);
            AssertSumOfEven(new[] { 1 }, 0);
            AssertSumOfEven(new[] { 2 }, 2);
            AssertSumOfEven(new[] { 0, 1, 2, 3, 4, 5, 6 }, 12);
        }

        static void Main(string[] args)
        {
            var result = SumOfEven(new[] { 0, 1, 2, 3, 4, 5, 6 });
            Console.WriteLine($"result: {result}");

            TestApi();
        }
    }
}
