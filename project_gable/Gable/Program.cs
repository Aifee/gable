using System;
using System.Runtime.InteropServices;
using Avalonia;
using Gable.CLI;
using Gable.GUI;

namespace Gable;

sealed class Program
{
    // 用于分配控制台
    [DllImport("kernel32.dll")]
    private static extern bool AllocConsole();

    // Initialization code. Don't use any Avalonia, third-party APIs or any
    // SynchronizationContext-reliant code before AppMain is called: things aren't initialized
    // yet and stuff might break.
    [STAThread]
    public static void Main(string[] args)
    {
        // 如果有参数，分配控制台
        if (args.Length > 0)
        {
            AllocConsole();
        }
        Console.WriteLine($"Gable Start:{string.Join(" ", args)}");
        Init();
        if (args.Length > 0)
        {
            CLIMain.Start(args);
        }
        else
        {
            GUIMain.Start(args);
        }
    }

    private static void Init()
    {
        // ModelManager.Instance.Init();
    }
}
