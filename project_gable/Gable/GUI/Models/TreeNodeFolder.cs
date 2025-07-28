using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Gable.Core.Enums;

namespace Gable.GUI.Models
{
    public class TreeNodeFolder : TreeNodeBase
    {
        public TreeNodeFolder(string fullPath)
            : base(ETreeItemType.FOLDER, fullPath) { }
    }
}
