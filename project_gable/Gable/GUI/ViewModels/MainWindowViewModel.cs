using System.Collections.ObjectModel;
using System.Diagnostics;
using Avalonia;
using Avalonia.Styling;
using CommunityToolkit.Mvvm.Input;
using Gable.GUI.Models;

namespace Gable.ViewModels;

public partial class MainWindowViewModel : ViewModelBase
{
    public ObservableCollection<Node> Nodes { get; }
    private Node? _selectedNode;
    public Node? SelectedNode
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
        Nodes = new ObservableCollection<Node>();
        Node root = new Node("Root");
        Nodes.Add(root);
        root.addNode(new Node("Folder 1"));
    }

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
        Debug.WriteLine("VIEWMODELS add folder");

        if (SelectedNode != null)
        {
            // 在指定节点下添加文件夹
            Node node = new Node("New Folder");
            SelectedNode.SubNodes.Add(node);
            SelectedNode = node;
        }
        else
        {
            // 添加到根节点
            Node node = new Node("New Folder");
            Nodes.Add(node);
            SelectedNode = node;
        }
    }

    [RelayCommand]
    private void AddFileNode()
    {
        Debug.WriteLine("VIEWMODELS add file");

        if (SelectedNode != null)
        {
            // 在指定节点下添加文件
            Node newFile = new Node("New File");
            SelectedNode.SubNodes.Add(newFile);
        }
        else
        {
            // 添加到根节点
            Node newFile = new Node("New File");
            Nodes.Add(newFile);
        }
    }

    [RelayCommand]
    private void DeleteNode()
    {
        Debug.WriteLine("VIEWMODELS delete node");

        if (SelectedNode != null)
        {
            // 从父节点中删除
            RemoveNode(Nodes, SelectedNode);
            SelectedNode = null;
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
