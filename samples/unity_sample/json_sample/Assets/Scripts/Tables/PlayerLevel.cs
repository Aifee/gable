using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    private Dictionary<int, PlayerLevel> _playerlevels;

    private void Load_PlayerLevel()
    {
        _playerlevels = new Dictionary<int, PlayerLevel>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/PlayerLevel");
        PlayerLevel[] array = LitJson.JsonMapper.ToObject<PlayerLevel[]>(asset.text);
        foreach (var item in array)
        {
            if (!_playerlevels.ContainsKey(item.id))
            {
                _playerlevels.Add(item.id, item);
            }
        }
    }

    public PlayerLevel GetPlayerLevel(int id)
    {
        if (_playerlevels.ContainsKey(id))
        {
            return _playerlevels[id];
        }
        return null;
    }
}

namespace Gable
{
    /// <summary>
    /// PlayerLevel 数据类
    /// </summary>
    public class PlayerLevel 
    {
        /// <summary>
        /// ID
        /// </summary>
        public int id;
        /// <summary>
        /// 攻击
        /// </summary>
        public int attack;
        /// <summary>
        /// 防御
        /// </summary>
        public int defense;
        /// <summary>
        /// 血量
        /// </summary>
        public long hp;
    }
}