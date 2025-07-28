using System;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO;
using System.Linq;
using Avalonia;
using Avalonia.Styling;
using CommunityToolkit.Mvvm.Input;
using Gable.Core.Common;
using Gable.Core.Enums;
using Gable.Core.Settings;
using Gable.GUI.Models;

namespace Gable.ViewModels;

public partial class MainWindowViewModel : ViewModelBase
{
    public ObservableCollection<TreeNodeBase> Nodes { get; } =
        new ObservableCollection<TreeNodeBase>();
    private TreeNodeBase? _selectedNode;
    public TreeNodeBase? SelectedNode
    {
        get => _selectedNode;
        set
        {
            _selectedNode = value;
            OnPropertyChanged("SelectedNode");
        }
    }

    public MainWindowViewModel()
    {
        ResetWorkspace();
    }

    #region Explorer TreeView

    /// <summary>
    /// 重置工作区
    /// </summary>
    private void ResetWorkspace()
    {
        Nodes.Clear();
        TraverseDirectory(GableSetting.BuildSettings.WorkspacePath);
    }

    /// <summary>
    /// 递归遍历指定目录下的所有文件和文件夹
    /// </summary>
    /// <param name="directoryPath">要遍历的目录路径</param>
    /// <param name="parentNode">父级树节点，如果为null则添加到根节点</param>
    private void TraverseDirectory(string directoryPath)
    {
        try
        {
            // 获取目录信息
            DirectoryInfo dirInfo = new DirectoryInfo(directoryPath);
            // 遍历所有子目录
            foreach (DirectoryInfo subDir in dirInfo.GetDirectories())
            {
                if (Global.IGNORE_PATH.Contains(subDir.Name))
                {
                    continue; // 跳过忽略的目录
                }
                AddTreeNode(ETreeItemType.FOLDER, subDir.FullName);
                // 递归遍历子目录
                TraverseDirectory(subDir.FullName);
            }

            // 遍历所有文件
            foreach (FileInfo file in dirInfo.GetFiles())
            {
                if (PathUtil.FiltrationSystemFiles(file.FullName))
                {
                    continue; // 跳过系统文件
                }
                if (
                    !file.Extension.Equals(
                        Global.GABLE_FILE_TYPE,
                        StringComparison.OrdinalIgnoreCase
                    )
                )
                {
                    continue; // 只处理特定类型的文件
                }
                if (!file.Name.Contains("@"))
                {
                    continue; // 跳过包含@的文件
                }
                string[] sprits = file.Name.Split("@");
                if (sprits.Length != 2)
                {
                    continue; // 跳过名称过短的文件
                }
                string alias = sprits[0];
                string fullPath = Path.Join(directoryPath, alias);
                if (!HasNode(fullPath))
                {
                    AddTreeNode(ETreeItemType.TABLE, fullPath);
                }
                AddTreeNode(ETreeItemType.SHEET, file.FullName);
            }
        }
        catch (Exception ex)
        {
            // 处理异常，如权限不足等
            Debug.WriteLine($"遍历目录时出错: {ex.Message}");
        }
    }

    private bool AddTreeNode(ETreeItemType type, string fullPath)
    {
        bool exists = HasNode(fullPath);
        if (exists)
        {
            return false;
        }
        string parentPath = PathUtil.GetParentPath(fullPath);
        TreeNodeBase? parent = GetNode(parentPath);
        TreeNodeBase? node = null;
        switch (type)
        {
            case ETreeItemType.FOLDER:
                node = new TreeNodeFolder(fullPath);
                break;
            case ETreeItemType.TABLE:
                node = new TreeNodeTable(fullPath);
                break;
            case ETreeItemType.SHEET:
                node = new TreeNodeSheet(fullPath);
                break;
        }
        if (node == null)
        {
            return false;
        }

        // 添加到根节点或选中节点下
        if (parent != null)
        {
            parent.AddSubNode(node);
        }
        else
        {
            Nodes.Add(node);
        }
        return true;
    }

    private bool HasNode(string fullPath)
    {
        if (Nodes.Count <= 0)
        {
            return false;
        }
        foreach (var node in Nodes)
        {
            if (fullPath == node.FullPath)
            {
                return true;
            }
        }
        return false;
    }

    private TreeNodeBase? GetNode(string fullPath)
    {
        if (!HasNode(fullPath))
        {
            return null;
        }
        return Nodes.FirstOrDefault(n =>
            n.FullPath.Equals(fullPath, StringComparison.OrdinalIgnoreCase)
        );
    }
    #endregion

    #region  Menu Commands
    [RelayCommand]
    private void Menu_NewFile() { /* 实现新建文件逻辑 */
    }

    [RelayCommand]
    private void Menu_NewFolder() { /* 实现新建文件夹逻辑 */
    }

    [RelayCommand]
    private void Menu_OpenProject() { /* 实现打开项目逻辑 */
    }

    [RelayCommand]
    private void Menu_Setting() { /* 实现打开设置逻辑 */
    }

    [RelayCommand]
    private void Menu_Exit() { /* 实现退出应用逻辑 */
    }

    [RelayCommand]
    private void Menu_CompileSetting() { /* 实现编译设置逻辑 */
    }

    [RelayCommand]
    private void Menu_QuickCompile() { /* 实现快速编译逻辑 */
    }

    [RelayCommand]
    private void Menu_ImportExcel() { /* 实现导入Excel逻辑 */
    }

    [RelayCommand]
    private void Menu_About() { /* 实现显示关于逻辑 */
    }

    [RelayCommand]
    private void Menu_ThemeFollowSystem()
    {
        OnSetTheme(EThemeType.Auto);
    }

    [RelayCommand]
    private void Menu_ThemeLight()
    {
        OnSetTheme(EThemeType.Light);
    }

    [RelayCommand]
    private void Menu_ThemeDark()
    {
        OnSetTheme(EThemeType.Dark);
    }

    private void OnSetTheme(EThemeType type)
    {
        if (Application.Current is not null)
        {
            switch (type)
            {
                case EThemeType.Auto:
                    Application.Current.RequestedThemeVariant = ThemeVariant.Default;
                    break;
                case EThemeType.Light:
                    Application.Current.RequestedThemeVariant = ThemeVariant.Light;
                    break;
                case EThemeType.Dark:
                    Application.Current.RequestedThemeVariant = ThemeVariant.Dark;
                    break;
            }
        }
    }
    #endregion

    [RelayCommand]
    private void AddFolderNode()
    {
        if (SelectedNode != null)
        {
            // 在指定节点下添加文件夹
            // TreeNodeBase node = new TreeNodeBase("New Folder");
            // SelectedNode.SubNodes.Add(node);
            // SelectedNode = node;
        }
        else
        {
            // 添加到根节点
            // Node node = new Node("New Folder");
            // Nodes.Add(node);
            // SelectedNode = node;
        }
    }

    [RelayCommand]
    private void AddFileNode()
    {
        Debug.WriteLine("VIEWMODELS add file");

        if (SelectedNode != null)
        {
            // 在指定节点下添加文件
            // Node newFile = new Node("New File");
            // SelectedNode.SubNodes.Add(newFile);
        }
        else
        {
            // 添加到根节点
            // Node newFile = new Node("New File");
            // Nodes.Add(newFile);
        }
    }

    [RelayCommand]
    private void DeleteNode()
    {
        Debug.WriteLine("VIEWMODELS delete node");

        if (SelectedNode != null)
        {
            // 从父节点中删除
            // RemoveNode(Nodes, SelectedNode);
            // SelectedNode = null;
        }
    }

    private bool RemoveNode(ObservableCollection<Node> nodes, Node nodeToRemove)
    {
        if (nodes.Remove(nodeToRemove))
        {
            return true;
        }

        foreach (var node in nodes)
        {
            if (RemoveNode(node.SubNodes, nodeToRemove))
            {
                return true;
            }
        }

        return false;
    }
}
