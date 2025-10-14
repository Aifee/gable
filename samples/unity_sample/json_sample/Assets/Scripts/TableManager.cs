using System.Collections;
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
            if(_instance == null)
            {
                _instance = new TableManager();
                _instance.Load();
            }
            return _instance;
        }
    }
    #endregion

    private void Load()
    {

    }
}
