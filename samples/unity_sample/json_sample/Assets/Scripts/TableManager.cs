using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
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


    public void Load()
    {
        Load_Const();
        Load_Player();
        Load_PlayerLevel();
    }
}