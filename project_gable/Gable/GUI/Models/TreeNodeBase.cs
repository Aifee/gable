using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Gable.Core.Common;
using Gable.Core.Enums;

namespace Gable.GUI.Models
{
    public class TreeNodeBase
    {
        /// <summary>
        /// 节点类型
        /// </summary>
        /// <value></value>
        public ETreeItemType TreeType { get; set; }

        /// <summary>
        /// 完整路径
        /// </summary>
        /// <value></value>
        public string FullPath { get; set; }

        /// <summary>
        /// 父节点
        /// </summary>
        /// <value></value>
        public TreeNodeBase? Parent { get; set; }

        /// <summary>
        /// 显示名
        /// </summary>
        /// <value></value>
        public string DisplayName { get; set; }

        /// <summary>
        /// 真实的文件名
        /// </summary>
        /// <value></value>
        public string FileName { get; set; }

        public ObservableCollection<TreeNodeBase> SubNodes { get; }

        public string IconPath
        {
            get
            {
                switch (TreeType)
                {
                    case ETreeItemType.FOLDER:
                        ESheetType st = PathUtil.PathToSheetType(FullPath);
                        if (st == ESheetType.DATA)
                        {
                            return Res.ICON_FOLDER_NORMAL;
                        }
                        else
                        {
                            return Res.ICON_FOLDER_SPECIAL;
                        }
                    case ETreeItemType.TABLE:
                        return Res.ICON_TABLE;
                    case ETreeItemType.SHEET:
                        return Res.ICON_SHEET;
                    default:
                        return Res.ICON_DEFUALT;
                }
            }
        }

        public TreeNodeBase(ETreeItemType treeType, string fullPath)
        {
            TreeType = treeType;
            FullPath = fullPath;
            Parent = null;
            DisplayName = PathUtil.GetBaseName(fullPath);
            FileName = fullPath;
            SubNodes = new ObservableCollection<TreeNodeBase>();
        }

        /// <summary>
        /// 添加子节点
        /// </summary>
        /// <param name="node"></param>
        public void AddSubNode(TreeNodeBase node)
        {
            if (node == null)
            {
                return;
            }
            node.Parent = this;
            SubNodes.Add(node);
        }

        /// <summary>
        /// 获取指定路径的节点
        /// </summary>
        /// <param name="fullPath"></param>
        /// <returns></returns>
        public TreeNodeBase? GetNode(string fullPath)
        {
            if (string.IsNullOrEmpty(fullPath))
            {
                return null;
            }

            if (FullPath == fullPath)
            {
                return this;
            }

            foreach (var subNode in SubNodes)
            {
                var foundNode = subNode.GetNode(fullPath);
                if (foundNode != null)
                {
                    return foundNode;
                }
            }
            return null;
        }

        public void Rename(string newName)
        {
            FullPath = Path.GetFullPath(newName);
        }
    }
}
