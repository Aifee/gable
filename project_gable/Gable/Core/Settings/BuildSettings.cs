using System.Collections.Generic;
using System.IO;
using Gable.Core.Common;

namespace Gable.Core.Settings
{
    public class BuildSettings
    {
        public string WorkspacePath { get; private set; } =
            Path.GetFullPath(PathUtil.GetApplicationDirectory());
        public List<BuildTarget> BuildTargets { get; set; } = new List<BuildTarget>();

        public BuildSettings()
        {
#if DEBUG
            var debugPath = Path.Join(
                PathUtil.GetApplicationDirectory(),
                "../../../../../../",
                "gable_project"
            );
            WorkspacePath = Path.GetFullPath(debugPath);
#endif
        }
    }
}
