using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Threading.Tasks;

namespace Gable.GUI.Models
{
    public class Node
    {
        public ObservableCollection<Node> SubNodes { get; }
        public string Title { get; }

        public Node(string title)
        {
            Title = title;
            SubNodes = new ObservableCollection<Node>();
        }

        public Node(string title, ObservableCollection<Node> subNodes)
        {
            Title = title;
            SubNodes = subNodes;
        }

        public void addNode(Node node)
        {
            SubNodes.Add(node);
        }
    }
}
