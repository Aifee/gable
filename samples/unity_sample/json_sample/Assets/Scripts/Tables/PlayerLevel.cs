using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    private Dictionary<int, Dictionary<int, PlayerLevel>> _playerlevels;

    private void Load_PlayerLevel()
    {
        _playerlevels = new Dictionary<int, Dictionary<int, PlayerLevel>>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/PlayerLevel");
        PlayerLevel[] array = LitJson.JsonMapper.ToObject<PlayerLevel[]>(asset.text);
        foreach (var item in array)
        {
            Dictionary<int, PlayerLevel> subItem;
            if(!_playerlevels.TryGetValue(item.id, out subItem))
            {
                subItem = new Dictionary<int, PlayerLevel>();
                _playerlevels.Add(item.id, subItem);
            }
            if (!subItem.ContainsKey(item.level))
            {
                subItem.Add(item.level, item);
            }
        }
    }

    public PlayerLevel GetPlayerLevel(int id, int level)
    {
        if (!_playerlevels.ContainsKey(id))
        {
            return null;
        }
        Dictionary<int, PlayerLevel> subItem = _playerlevels[id];
        if (subItem.ContainsKey(level))
        {
            return subItem[level];
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
        /// 等级
        /// </summary>
        public int level;
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