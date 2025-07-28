using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Gable.Core.Enums;

namespace Gable.GUI.Models
{
    public class TreeNodeTable : TreeNodeBase
    {
        public TreeNodeTable(string fullPath)
            : base(ETreeItemType.TABLE, fullPath) { }
    }
}
