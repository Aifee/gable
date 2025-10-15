// {{CLASS_NAME}}.ts

{%- for import in imports %}
import { {{ import }} } from './{{ import }}';
{%- endfor %}

/**
 * {{CLASS_NAME}}
 */
export class {{CLASS_NAME}} {
    {%- for field in info.fields %}
    /**
     * {{field.field_desc}}
     */
    {{ field.field_name }}: {{ field.field_type }} = null!;
    {%- endfor %}
    
    constructor() {
        {%- for field in info.fields %}
        this.{{ field.field_name }} = null!;
        {%- endfor %}
    }
}