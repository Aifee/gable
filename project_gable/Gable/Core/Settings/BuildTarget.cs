using Gable.Core.Enums;

namespace Gable.Core.Settings
{
    public class BuildTarget
    {
        public EDevelopType DevelopType;
        public bool Enable;
        public string? DisplayName;
        public string? Keyword;
        public ETargetFormat Target;
        public string? TargetPath;
        public bool GenerateScript;
        public string? ScriptPath;
    }
}
