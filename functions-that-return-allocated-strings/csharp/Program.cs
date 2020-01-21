using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;

// We follow a similar pattern to the object example: the Rust string is
// contained within a subclass of SafeHandle and a wrapper class ThemeSong
// ensures that the handle is disposed properly.
//
// Unfortunately, there is no easy way to read the pointer as a UTF-8 string.
// C# has cases for ANSI strings and for "Unicode" strings,
// but nothing for UTF-8. We need to write that ourselves.
namespace Client
{
    internal class Native
    {
        [DllImport("api", EntryPoint = "generate_theme_song")]
        internal static extern ThemeSongHandle GenerateThemeSong(byte length);

        [DllImport("api", EntryPoint = "free_theme_song")]
        internal static extern void FreeThemeSong(IntPtr song);
    }

    internal class ThemeSongHandle : SafeHandle
    {
        public ThemeSongHandle() : base(IntPtr.Zero, true) { }

        public override bool IsInvalid
        {
            get { return false; }
        }

        public string AsString()
        {
            int len = 0;
            while (Marshal.ReadByte(handle, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);

            return Encoding.UTF8.GetString(buffer);
        }

        protected override bool ReleaseHandle()
        {
            Native.FreeThemeSong(handle);

            return true;
        }
    }

    public class ThemeSong : IDisposable
    {
        private ThemeSongHandle _song;
        private string _songString;

        public ThemeSong(byte length)
        {
            _song = Native.GenerateThemeSong(length);
        }

        public override string ToString()
        {
            if (_songString == null)
            {
                _songString = _song.AsString();
            }

            return _songString;
        }

        public void Dispose()
        {
            _song.Dispose();
        }
    }

    class Program
    {
        static void TestApi()
        {
            void AssertGenerateThemeSong(byte length, string expectedResult)
            {
                using var result = new ThemeSong(length);

                Debug.Assert(expectedResult == result.ToString());
            }

            AssertGenerateThemeSong(0, "💣 Batman! 💣");
            AssertGenerateThemeSong(1, "💣 na Batman! 💣");
            AssertGenerateThemeSong(2, "💣 na na Batman! 💣");
        }

        static void Main(string[] args)
        {
            var song = new ThemeSong(5);
            Console.WriteLine("{0}", song);

            TestApi();
        }
    }
}
