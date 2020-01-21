using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace Client
{
    [StructLayout(LayoutKind.Sequential)]
    struct ResultTuple
    {
        public uint sum;
        public uint errCode;

        public static implicit operator Tuple<uint, uint>(ResultTuple result)
        {
            return Tuple.Create(result.sum, result.errCode);
        }

        public static implicit operator ResultTuple(Tuple<uint, uint> result)
        {
            return new ResultTuple { sum = result.Item1, errCode = result.Item2 };
        }
    };

    class Program
    {
        [DllImport("api", EntryPoint = "addition")]
        private static extern ResultTuple Addition(uint a, uint b);

        static void TestApi()
        {
            void AssertAddition(uint a, uint b, (uint, uint) expectedResult)
            {
                var result = Addition(a, b);

                Debug.Assert(expectedResult.Item1 == result.sum);
                Debug.Assert(expectedResult.Item2 == result.errCode);
            }

            AssertAddition(0, 0, (0, 0));
            AssertAddition(0, 1, (1, 0));
            AssertAddition(0, uint.MaxValue, (uint.MaxValue, 0));
            AssertAddition(1, 0, (1, 0));
            AssertAddition(1, 1, (2, 0));
            AssertAddition(1, uint.MaxValue - 1, (uint.MaxValue, 0));
            AssertAddition(1, uint.MaxValue, (0, 1));
        }

        static void Main(string[] args)
        {
            var result = Addition(1, 2);
            Console.WriteLine($"sum: {result.sum}, errCode: {result.errCode}");

            TestApi();
        }
    }
}
