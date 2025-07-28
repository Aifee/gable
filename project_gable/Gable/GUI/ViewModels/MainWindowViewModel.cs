using System.Collections.ObjectModel;
using System.Diagnostics;
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
