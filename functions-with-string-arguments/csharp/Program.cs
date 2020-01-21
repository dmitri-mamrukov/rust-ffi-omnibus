using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace Client
{
    class Program
    {
        [DllImport("api", EntryPoint = "how_many_characters")]
        public static extern int HowManyCharacters(string text);

        static void TestApi()
        {
            void AssertHowManyCharacters(string text, int expectedResult)
            {
                var result = HowManyCharacters(text);

                Debug.Assert(expectedResult == result);
            }

            AssertHowManyCharacters(null, -1);
            AssertHowManyCharacters("", 0);
            AssertHowManyCharacters("rust", 4);
        }

        static void Main(string[] args)
        {
            var result = HowManyCharacters("rust");
            Console.WriteLine($"count: {result}");

            TestApi();
        }
    }
}
