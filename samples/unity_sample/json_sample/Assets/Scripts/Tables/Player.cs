using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    private Dictionary<int, Dictionary<int, Player>> _players;

    private void Load_Player()
    {
        _players = new Dictionary<int, Dictionary<int,Player>>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/Player");
        Player[] array = LitJson.JsonMapper.ToObject<Player[]>(asset.text);
        foreach (var item in array)
        {
            Dictionary<int, Player> subItem;
            if(!_players.TryGetValue(item.id, out subItem))
            {
                subItem = new Dictionary<int, Player>();
                _players.Add(item.id, subItem);
            }
            if (!subItem.ContainsKey(item.cd))
            {
                subItem.Add(item.cd, item);
            }
        }
    }

    public Player GetPlayer(int id, int cd)
    {
        if (!_players.ContainsKey(id))
        {
            return  null;
        }
        Dictionary<int, Player> subItem = _players[id];
        if (subItem.ContainsKey(cd))
        {
            return subItem[cd];
        }
        return null;
    }
}

namespace Gable
{
    // 这是一个测试
    public class Player 
    {
        /// <summary>
        /// ID
        /// </summary>
        public int id;
        /// <summary>
        /// 类型
        /// </summary>
        public EPlayerType type;
        /// <summary>
        /// 名字
        /// </summary>
        public string name;
        /// <summary>
        /// 等级
        /// </summary>
        public int level;
        /// <summary>
        /// 升级消耗时间
        /// </summary>
        public int cd;
    }
}