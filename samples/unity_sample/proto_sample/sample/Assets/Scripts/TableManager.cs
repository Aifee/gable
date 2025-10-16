using Gable;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

public class TableManager
{
    #region Instance

    private static TableManager _instance;

    public static TableManager Instance
    {
        get
        {
            if (_instance == null)
            {
                _instance = new TableManager();
                _instance.Load();
            }
            return _instance;
        }
    }

    #endregion Instance

    #region Table Const

    private Const _const;

    public Const Const
    { get { return _const; } }

    private void Load_Const()
    {
        TextAsset asset = Resources.Load<TextAsset>("Tables/Const");
        using (MemoryStream msg = new MemoryStream(asset.bytes))
        {
            _const = ProtoBuf.Serializer.Deserialize<Const>(msg);
        }
    }

    #endregion Table Const

    #region Table Player

    private Dictionary<int, Player> _players;

    private void Load_Player()
    {
        _players = new Dictionary<int, Player>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/Player");
        using (MemoryStream msg = new MemoryStream(asset.bytes))
        {
            Player[] array = ProtoBuf.Serializer.Deserialize<Player[]>(msg);
            foreach (Player player in array)
            {
                if (!_players.ContainsKey(player.id))
                {
                    _players.Add(player.id, player);
                }
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

    #endregion Table Player

    #region PlayerLevel
    private Dictionary<int, Dictionary<int, PlayerLevel>> _playerLevel;

    private void Load_PlayerLevel()
    {
        _playerLevel = new Dictionary<int, Dictionary<int, PlayerLevel>>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/PlayerLevel");
        using (MemoryStream msg = new MemoryStream(asset.bytes))
        {
            PlayerLevel[] array = ProtoBuf.Serializer.Deserialize<PlayerLevel[]>(msg);
            foreach (PlayerLevel level in array)
            {
                Dictionary<int, PlayerLevel> subItem;
                if(!_playerLevel.TryGetValue(level.id, out subItem))
                {
                    subItem = new Dictionary<int, PlayerLevel>();
                    _playerLevel.Add(level.id, subItem);
                }
                if (!subItem.ContainsKey(level.level))
                {
                    subItem.Add(level.level, level);
                }
            }
        }
    }

    public PlayerLevel GetPlayerLevel(int id, int level)
    {
        Dictionary<int, PlayerLevel> subItem;
        if (!_playerLevel.TryGetValue(id, out subItem))
        {
            return null;
        }
        if (subItem.ContainsKey(level))
        {
            return subItem[level];
        }
        return null;
    }

    #endregion

    public void Load()
    {
        Load_Const();
        Load_Player();
        Load_PlayerLevel();
    }
}