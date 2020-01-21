using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace Client
{
    internal class Native
    {
        [DllImport("api", EntryPoint = "zip_code_database_new")]
        internal static extern ZipCodeDatabaseHandle ZipCodeDatabaseNew();

        [DllImport("api", EntryPoint = "zip_code_database_free")]
        internal static extern void ZipCodeDatabaseFree(IntPtr db);

        [DllImport("api", EntryPoint = "zip_code_database_populate")]
        internal static extern void ZipCodeDatabasePopulate(ZipCodeDatabaseHandle db);

        [DllImport("api", EntryPoint = "zip_code_database_population_of")]
        internal static extern uint ZipCodeDatabasePopulationOf(ZipCodeDatabaseHandle db, string zipCode);
    }

    internal class ZipCodeDatabaseHandle : SafeHandle
    {
        public ZipCodeDatabaseHandle() : base(IntPtr.Zero, true) { }

        public override bool IsInvalid
        {
            get { return false; }
        }

        protected override bool ReleaseHandle()
        {
            Native.ZipCodeDatabaseFree(handle);

            return true;
        }
    }

    public class ZipCodeDatabase : IDisposable
    {
        private ZipCodeDatabaseHandle _db;

        public ZipCodeDatabase()
        {
            _db = Native.ZipCodeDatabaseNew();
        }

        public void Populate()
        {
            Native.ZipCodeDatabasePopulate(_db);
        }

        public uint PopulationOf(string zipCode)
        {
            return Native.ZipCodeDatabasePopulationOf(_db, zipCode);
        }

        public void Dispose()
        {
            _db.Dispose();
        }
    }

    class Program
    {
        static void TestApi()
        {
            {
                using var db = new ZipCodeDatabase();
                db.Populate();
            }
            {
                using var db = new ZipCodeDatabase();
                db.Populate();

                var result = db.PopulationOf("00000");

                Debug.Assert(0 == result);

                result = db.PopulationOf("90210");

                Debug.Assert(90210 == result);

                result = db.PopulationOf("20500");

                Debug.Assert(20500 == result);

                result = db.PopulationOf("99999");

                Debug.Assert(99999 == result);

                result = db.PopulationOf("non-existing-zip-code");

                Debug.Assert(0 == result);
            }
        }

        static void Main(string[] args)
        {
            using var db = new ZipCodeDatabase();
            db.Populate();

            var pop1 = db.PopulationOf("90210");
            var pop2 = db.PopulationOf("20500");

            Console.WriteLine($"pop1: {pop1}, pop2: {pop2}");

            TestApi();
        }
    }
}
