using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    private Dictionary<int, Player> _players;

    private void Load_Player()
    {
        _players = new Dictionary<int, Player>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/Player");
        Player[] array = LitJson.JsonMapper.ToObject<Player[]>(asset.text);
        foreach (var item in array)
        {
            if (!_players.ContainsKey(item.id))
            {
                _players.Add(item.id, item);
            }
        }
    }

    public Player GetPlayer(int id)
    {
        if (_players.ContainsKey(id))
        {
            return _players[id];
        }
        return null;
    }
}

namespace Gable
{
    /// <summary>
    /// Player 数据类
    /// </summary>
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