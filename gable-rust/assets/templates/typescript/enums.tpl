// {{CLASS_NAME}}.ts

/**
 * {{CLASS_NAME}}
 */
export enum {{CLASS_NAME}} {
    {%- for field in fields %}
    /**
     * {{field.field_desc}}
     */
    {{ field.field_name }} = {{ field.field_index }}{% if not loop.last %},{% endif %}
    {%- endfor %}
}