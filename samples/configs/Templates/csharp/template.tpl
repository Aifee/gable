using UnityEngine;

namespace Gable
{
    // 这是一个测试
    public class {{CLASS_NAME}} 
    {
        {%- for field in fields %}
        /// <summary>
        /// {{field.field_desc}}
        /// </summary>
        public {{ field.field_type }} {{ field.field_name }};
        {%- endfor %}
    }
}