using UnityEngine;

namespace Gable
{
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