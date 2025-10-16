using Gable;
using System.Collections.Generic;
using UnityEngine;

public partial class TableManager
{
    {% if info.primary_num == 0 -%}
    private {{CLASS_NAME}} _{{CLASS_NAME | lower}};
    public {{CLASS_NAME}} {{CLASS_NAME}} => _{{CLASS_NAME | lower}};

    private void Load_{{CLASS_NAME}}()
    {
        TextAsset asset = Resources.Load<TextAsset>("Tables/{{CLASS_NAME}}");
        _{{CLASS_NAME | lower}} = LitJson.JsonMapper.ToObject<{{CLASS_NAME}}>(asset.text);
    }
    {%- elif info.primary_num == 1 -%}
    private Dictionary<{{info.main_fields[0].field_type}}, {{CLASS_NAME}}> _{{CLASS_NAME | lower}}s;

    private void Load_{{CLASS_NAME}}()
    {
        _{{CLASS_NAME | lower}}s = new Dictionary<{{info.main_fields[0].field_type}}, {{CLASS_NAME}}>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/{{CLASS_NAME}}");
        {{CLASS_NAME}}[] array = LitJson.JsonMapper.ToObject<{{CLASS_NAME}}[]>(asset.text);
        foreach (var item in array)
        {
            if (!_{{CLASS_NAME | lower}}s.ContainsKey(item.{{info.main_fields[0].field_name}}))
            {
                _{{CLASS_NAME | lower}}s.Add(item.{{info.main_fields[0].field_name}}, item);
            }
        }
    }

    public {{CLASS_NAME}} Get{{CLASS_NAME}}({{info.main_fields[0].field_type}} {{info.main_fields[0].field_name}})
    {
        if (_{{CLASS_NAME | lower}}s.ContainsKey({{info.main_fields[0].field_name}}))
        {
            return _{{CLASS_NAME | lower}}s[{{info.main_fields[0].field_name}}];
        }
        return null;
    }
    {%- else -%}
    private Dictionary<{{info.main_fields[0].field_type}}, Dictionary<{{info.main_fields[1].field_type}}, {{CLASS_NAME}}>> _{{CLASS_NAME | lower}}s;

    private void Load_{{CLASS_NAME}}()
    {
        _{{CLASS_NAME | lower}}s = new Dictionary<{{info.main_fields[0].field_type}}, Dictionary<{{info.main_fields[1].field_type}}, {{CLASS_NAME}}>>();
        TextAsset asset = Resources.Load<TextAsset>("Tables/{{CLASS_NAME}}");
        {{CLASS_NAME}}[] array = LitJson.JsonMapper.ToObject<{{CLASS_NAME}}[]>(asset.text);
        foreach (var item in array)
        {
            Dictionary<{{info.main_fields[1].field_type}}, {{CLASS_NAME}}> subItem;
            if(!_{{CLASS_NAME | lower}}s.TryGetValue(item.{{info.main_fields[0].field_name}}, out subItem))
            {
                subItem = new Dictionary<{{info.main_fields[1].field_type}}, {{CLASS_NAME}}>();
                _{{CLASS_NAME | lower}}s.Add(item.{{info.main_fields[0].field_name}}, subItem);
            }
            if (!subItem.ContainsKey(item.{{info.main_fields[1].field_name}}))
            {
                subItem.Add(item.{{info.main_fields[1].field_name}}, item);
            }
        }
    }

    public {{CLASS_NAME}} Get{{CLASS_NAME}}({{info.main_fields[0].field_type}} {{info.main_fields[0].field_name}}, {{info.main_fields[1].field_type}} {{info.main_fields[1].field_name}})
    {
        if (!_{{CLASS_NAME | lower}}s.ContainsKey({{info.main_fields[0].field_name}}))
        {
            return null;
        }
        Dictionary<{{info.main_fields[1].field_type}}, {{CLASS_NAME}}> subItem = _{{CLASS_NAME | lower}}s[{{info.main_fields[0].field_name}}];
        if (subItem.ContainsKey({{info.main_fields[1].field_name}}))
        {
            return subItem[{{info.main_fields[1].field_name}}];
        }
        return null;
    }
    {%- endif %}
}

namespace Gable
{
    /// <summary>
    /// {{CLASS_NAME}} 数据类
    /// </summary>
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