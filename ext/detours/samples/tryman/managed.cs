using System;
using System.Reflection;
using System.Runtime.InteropServices;

[assembly: AssemblyProduct("Microsoft Research Detours")]
[assembly: AssemblyCompany("Microsoft Corporation")]
[assembly: AssemblyVersion("1.0.0.0")]

public class Test
{
    //    [DllImport("kernel32.dll", CharSet=CharSet.Auto, SetLastError=true)]
    //    static extern IntPtr LoadLibrary([In, MarshalAs(UnmanagedType.LPStr)] string lpFileName);

    [DllImport("kernel32", CharSet=CharSet.Auto, SetLastError=true)]
    static extern IntPtr LoadLibrary(string lpFileName);

    public static int Main()
    {
        if (IntPtr.Size == 4) {
            Console.WriteLine("  *** Managed code with 32-bit runtime ({0})",
                              Environment.Version);
        }
        else if (IntPtr.Size == 8) {
            Console.WriteLine("  *** Managed code with 64-bit runtime ({0})",
                              Environment.Version);
        }
        else {
            Console.WriteLine("  *** Managed code of unknown IntPtr.Size: {0}", IntPtr.Size);
        }

        if (IntPtr.Size == 4) {
            if (LoadLibrary("tstman32.dll") == (IntPtr)0) {
                Console.WriteLine("--------: managed code failed to load tstman32.dll");

            }
        }
        else {
            if (LoadLibrary("tstman64.dll") == (IntPtr)0) {
                Console.WriteLine("--------: managed code failed to load tstman64.dll");

            }
        }

        return 0;
    }
}


