using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Gable.Core.Enums;

namespace Gable.GUI.Models
{
    public class TreeNodeSheet : TreeNodeBase
    {
        public TreeNodeSheet(string fullPath)
            : base(ETreeItemType.SHEET, fullPath) { }
    }
}
