using UnityEngine;

namespace Gable
{
    public enum {{CLASS_NAME}} 
    {
        {%- for field in info.fields %}
        /// <summary>
        /// {{field.field_desc}}
        /// </summary>
        {{ field.field_name }} = {{ field.field_index }},
        {%- endfor %}
    }
}