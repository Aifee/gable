using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    private Const _const;
    public Const Const => _const;

    private void Load_Const()
    {
        TextAsset asset = Resources.Load<TextAsset>("Tables/Const");
        _const = LitJson.JsonMapper.ToObject<Const>(asset.text);
    }
}

namespace Gable
{
    /// <summary>
    /// Const 数据类
    /// </summary>
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