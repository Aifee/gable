using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Gable.Core.Enums;

namespace Gable.Core.Common
{
    public static class Utils
    {
        public static string GetKeyword(EDevelopType type)
        {
            return type switch
            {
                EDevelopType.C => "cpp",
                EDevelopType.CSHARP => "cs",
                EDevelopType.CANGJIE => "cj",
                EDevelopType.GO => "go",
                EDevelopType.JAVA => "java",
                EDevelopType.JAVASCRIPT => "js",
                EDevelopType.LUA => "lua",
                EDevelopType.PYTHON => "py",
                EDevelopType.TYPESCRIPT => "ts",
                _ => string.Empty,
            };
        }
    }
}
