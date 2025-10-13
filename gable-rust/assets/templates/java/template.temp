package gable;

{%- for import in imports %}
import {{ import }};
{%- endfor %}
/**
 * {{CLASS_NAME}}
 */
public class {{CLASS_NAME}} {
    {%- for field in fields %}
    /**
     * {{field.field_desc}}
     */
    public {{ field.field_type }} {{ field.field_name }};
    {%- endfor %}
}