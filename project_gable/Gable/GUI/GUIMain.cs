using Avalonia;

namespace Gable.GUI
{
    public static class GUIMain
    {
        public static void Start(string[] args)
        {
            AppBuilder.Configure<App>()
                .UsePlatformDetect()
                .WithInterFont()
                .LogToTrace()
                .StartWithClassicDesktopLifetime(args);
        }
    }
}