using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    private Const _consts;

    private void Load_Const()
    {
        TextAsset asset = Resources.Load<TextAsset>("Tables/Const");
        _consts = LitJson.JsonMapper.ToObject<Const>(asset.text);
    }

    public Const GetConst(int id)
    {
        if (_consts.ContainsKey(id))
        {
            return _consts[id];
        }
        return null;
    }
}

namespace Gable
{
    // 这是一个测试
    public class Const 
    {
        /// <summary>
        /// 默认玩家坐标
        /// </summary>
        public Vector3 player_default_position;
        /// <summary>
        /// 地图默认高度
        /// </summary>
        public float map_height;
    }
}