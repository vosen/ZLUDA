using System;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;

namespace Zluda
{
    class Program
    {
        [UnmanagedFunctionPointer(CallingConvention.Winapi)]
        private delegate int CuInit(int flags);

        static int Main(string[] args)
        {
            DirectoryInfo exeDirectory = Directory.GetParent(Assembly.GetEntryAssembly().Location);
            string dllPath = Path.Combine(exeDirectory.ToString(), "do_cuinit.dll");
            IntPtr nvcuda = NativeMethods.LoadLibrary(dllPath);
            if (nvcuda == IntPtr.Zero)
                return 1;
            IntPtr doCuinitPtr = NativeMethods.GetProcAddress(nvcuda, "do_cuinit");
            CuInit cuinit = (CuInit)Marshal.GetDelegateForFunctionPointer(doCuinitPtr, typeof(CuInit));
            return cuinit(0);
        }
    }

    static class NativeMethods
    {
        [DllImport("kernel32.dll")]
        public static extern IntPtr LoadLibrary(string dllToLoad);

        [DllImport("kernel32.dll")]
        public static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);
    }
}
