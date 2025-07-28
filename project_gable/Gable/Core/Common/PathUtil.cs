using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Gable.Core.Settings;

namespace Gable.Core.Common
{
    public static class PathUtil
    {
        /// <summary>
        /// 获取目录的上一级目录
        /// 如果不在工作区目录下，则返回工作区目录
        /// </summary>
        /// <param name="fullPath"></param>
        /// <returns></returns> <summary>
        public static string GetParentPath(string fullPath)
        {
            if (string.IsNullOrEmpty(fullPath))
            {
                return GableSetting.BuildSettings.WorkspacePath;
            }
            if (fullPath.Contains("@"))
            {
                int index = fullPath.IndexOf("@");
                if (index > 0)
                {
                    return fullPath.Substring(0, index);
                }
            }

            var directoryInfo = new DirectoryInfo(fullPath);
            if (directoryInfo.Parent == null)
            {
                return GableSetting.BuildSettings.WorkspacePath;
            }
            if (!directoryInfo.Parent.FullName.Contains(GableSetting.BuildSettings.WorkspacePath))
            {
                return GableSetting.BuildSettings.WorkspacePath;
            }
            return directoryInfo.Parent.FullName;
        }

        public static string GetBaseName(string fullPath)
        {
            if (string.IsNullOrEmpty(fullPath))
            {
                return string.Empty;
            }
            string basename = Path.GetFileName(fullPath);
            return basename;
        }

        /// <summary>
        /// 获取应用程序所在目录
        /// </summary>
        /// <returns></returns>
        public static string GetApplicationDirectory()
        {
            return AppDomain.CurrentDomain.BaseDirectory;
        }

        /// <summary>
        /// 过滤系统文件
        /// </summary>
        /// <param name="fullPath"></param>
        /// <returns></returns>
        public static bool FiltrationSystemFiles(string fullPath)
        {
            if (string.IsNullOrEmpty(fullPath))
            {
                return false;
            }

            var fileInfo = new FileInfo(fullPath);
            if (
                fileInfo.Attributes.HasFlag(FileAttributes.Hidden)
                || fileInfo.Attributes.HasFlag(FileAttributes.System)
            )
            {
                return true;
            }

            return false;
        }
    }
}
