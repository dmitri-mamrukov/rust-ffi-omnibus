using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

/// <summary>
/// To mirror the tuple structure definition, we create a struct using the
/// StructLayout property and define the layout as sequential. We also provide
/// some implicit conversion operators to make going between types fairly
/// seamless.
/// </summary>
namespace Client
{
    [StructLayout(LayoutKind.Sequential)]
    struct IntTuple
    {
        public uint x;
        public uint y;

        public static implicit operator Tuple<uint, uint>(IntTuple tuple)
        {
            return Tuple.Create(tuple.x, tuple.y);
        }

        public static implicit operator IntTuple(Tuple<uint, uint> tuple)
        {
            return new IntTuple { x = tuple.Item1, y = tuple.Item2 };
        }
    };

    class Program
    {
        [DllImport("api", EntryPoint = "flip_things_around")]
        private static extern IntTuple FlipThingsAround(IntTuple tuple);

        static void TestApi()
        {
            void AssertFlipThingsAround(IntTuple tuple, IntTuple expectedResult)
            {
                var result = FlipThingsAround(tuple);

                Debug.Assert(expectedResult.x == result.x);
                Debug.Assert(expectedResult.y == result.y);
            }

            AssertFlipThingsAround(
                new IntTuple { x = 0, y = 0 },
                new IntTuple { x = 1, y = uint.MaxValue });
            AssertFlipThingsAround(
                new IntTuple { x = 0, y = 2 },
                new IntTuple { x = 3, y = uint.MaxValue });
            AssertFlipThingsAround(
                new IntTuple { x = 0, y = uint.MaxValue },
                new IntTuple { x = 0, y = uint.MaxValue });
            AssertFlipThingsAround(
                new IntTuple { x = 1, y = uint.MaxValue },
                new IntTuple { x = 0, y = 0 });
            AssertFlipThingsAround(
                new IntTuple { x = 1, y = 2 },
                new IntTuple { x = 3, y = 0 });
            AssertFlipThingsAround(
                new IntTuple { x = 1, y = uint.MaxValue },
                new IntTuple { x = 0, y = 0 });
            AssertFlipThingsAround(
                new IntTuple { x = uint.MaxValue, y = 0 },
                new IntTuple { x = 1, y = uint.MaxValue - 1 });
            AssertFlipThingsAround(
                new IntTuple { x = uint.MaxValue, y = 2 },
                new IntTuple { x = 3, y = uint.MaxValue - 1 });
            AssertFlipThingsAround(
                new IntTuple { x = uint.MaxValue, y = uint.MaxValue },
                new IntTuple { x = 0, y = uint.MaxValue - 1 });
        }

        static void Main(string[] args)
        {
            var result = FlipThingsAround(new IntTuple { x = 1, y = 2 });
            Console.WriteLine($"x: {result.x}, y: {result.y}");

            TestApi();
        }
    }
}
