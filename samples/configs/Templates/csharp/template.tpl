using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    private Dictionary<int, {{CLASS_NAME}}> _{{CLASS_NAME | lower}}s;

    private void Load_{{CLASS_NAME}}()
    {
        _{{CLASS_NAME | lower}}s = new Dictionary<int, {{CLASS_NAME}}>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/{{CLASS_NAME}}");
        {{CLASS_NAME}}[] array = LitJson.JsonMapper.ToObject<{{CLASS_NAME}}[]>(asset.text);
        foreach (var item in array)
        {
            if (!_{{CLASS_NAME | lower}}s.ContainsKey(item.id))
            {
                _{{CLASS_NAME | lower}}s.Add(item.id, item);
            }
        }
    }

    public {{CLASS_NAME}} Get{{CLASS_NAME}}(int id)
    {
        if (_{{CLASS_NAME | lower}}s.ContainsKey(id))
        {
            return _{{CLASS_NAME | lower}}s[id];
        }
        return null;
    }
}

namespace Gable
{
    // 这是一个测试
    public class {{CLASS_NAME}} 
    {
        {%- for field in info.fields %}
        /// <summary>
        /// {{field.field_desc}}
        /// </summary>
        public {{ field.field_type }} {{ field.field_name }};
        {%- endfor %}
    }
}