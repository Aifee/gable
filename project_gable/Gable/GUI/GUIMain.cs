using Avalonia;
using Gable.Core.Settings;

namespace Gable.GUI
{
    public static class GUIMain
    {
        public static void Start(string[] args)
        {
            GableSetting.InitBuildSettings();
            AppBuilder
                .Configure<App>()
                .UsePlatformDetect()
                .WithInterFont()
                .LogToTrace()
                .StartWithClassicDesktopLifetime(args);
        }
    }
}
