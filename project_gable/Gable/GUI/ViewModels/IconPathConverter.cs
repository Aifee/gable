using System;
using System.Globalization;
using Avalonia.Data.Converters;
using Avalonia.Media.Imaging;
using Avalonia.Platform;

namespace Gable.GUI.ViewModels
{
    public class IconPathConverter : IValueConverter
    {
        public static readonly IconPathConverter Instance = new IconPathConverter();

        public object? Convert(
            object? value,
            Type targetType,
            object? parameter,
            CultureInfo culture
        )
        {
            if (value is string path && !string.IsNullOrEmpty(path))
            {
                try
                {
                    // 使用 Avalonia 提供的静态方法加载资源
                    var uri = new Uri($"avares://Gable{path}");
                    return new Bitmap(AssetLoader.Open(uri));
                }
                catch
                {
                    // 如果无法加载指定图标，返回默认图标或null
                    return GetDefaultIcon();
                }
            }
            return GetDefaultIcon();
        }

        public object ConvertBack(
            object? value,
            Type targetType,
            object? parameter,
            CultureInfo culture
        )
        {
            throw new NotImplementedException();
        }

        private object? GetDefaultIcon()
        {
            try
            {
                var uri = new Uri($"avares://Gable/Assets/Icon/error.ico");
                return new Bitmap(AssetLoader.Open(uri));
            }
            catch
            {
                return null;
            }
        }
    }
}
