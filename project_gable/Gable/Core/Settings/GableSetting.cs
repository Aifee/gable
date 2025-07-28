using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;
using Gable.Core.Common;

namespace Gable.Core.Settings
{
    public static class GableSetting
    {
        public static bool IS_CLI { get; private set; } = false;

        public static BuildSettings BuildSettings { get; private set; } = new BuildSettings();

        public static void SetIsCli(bool isCli)
        {
            IS_CLI = isCli;
        }

        public static void InitBuildSettings()
        {
            string settingFile = Path.Join(
                PathUtil.GetApplicationDirectory(),
                Global.PROJECT_DATA_DIR,
                Global.SETTING_PREFS
            );
            if (Path.Exists(settingFile))
            {
                string content = File.ReadAllText(settingFile);
                if (!string.IsNullOrEmpty(content))
                {
                    var deserializedSettings = JsonSerializer.Deserialize<BuildSettings>(content);
                    if (deserializedSettings != null)
                    {
                        BuildSettings = deserializedSettings;
                    }
                }
            }
        }
    }
}
