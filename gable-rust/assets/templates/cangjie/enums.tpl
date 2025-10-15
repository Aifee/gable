module gable;

{%- for import in imports %}
import {{ import }};
{%- endfor %}

/**
 * {{CLASS_NAME}}
 */
class {{CLASS_NAME}} {
    {%- for field in info.fields %}
    /**
     * {{field.field_desc}}
     */
    public {{ field.field_name }}: {{ field.field_type }} = default;
    {%- endfor %}
}