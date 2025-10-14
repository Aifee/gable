using Gable;
using System.Collections.Generic;
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
        _const = LitJson.JsonMapper.ToObject<Const>(asset.text);
    }

    #endregion Table Const

    #region Table Player

    private Dictionary<int, Player> _players;

    private void Load_Player()
    {
        _players = new Dictionary<int, Player>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/Player");
        Player[] array = LitJson.JsonMapper.ToObject<Player[]>(asset.text);
        foreach (Player player in array)
        {
            if (!_players.ContainsKey(player.id))
            {
                _players.Add(player.id, player);
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
    private Dictionary<int, PlayerLevel> _playerLevel;

    private void Load_PlayerLevel()
    {
        _playerLevel = new Dictionary<int, PlayerLevel>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/PlayerLevel");
        PlayerLevel[] array = LitJson.JsonMapper.ToObject<PlayerLevel[]>(asset.text);
        foreach (PlayerLevel level in array)
        {
            if (!_playerLevel.ContainsKey(level.id))
            {
                _playerLevel.Add(level.id, level);
            }
        }
    }

    public PlayerLevel GetPlayerLevel(int id)
    {
        if (_playerLevel.ContainsKey(id))
        {
            return _playerLevel[id];
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