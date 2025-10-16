// {{CLASS_NAME}}.js

{%- for import in imports %}
import { {{ import }} } from './{{ import }}.js';
{%- endfor %}

/**
 * {{CLASS_NAME}}
 */
export class {{CLASS_NAME}} {
    {%- for field in info.fields %}
    /**
     * {{field.field_desc}}
     */
    {{ field.field_name }} = null;
    {%- endfor %}
    
    constructor() {
        {%- for field in info.fields %}
        this.{{ field.field_name }} = null;
        {%- endfor %}
    }
}