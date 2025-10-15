module gable;

/**
 * {{CLASS_NAME}}
 */
enum {{CLASS_NAME}} {
    {%- for field in info.fields %}
    /**
     * {{field.field_desc}}
     */
    {{ field.field_name }} = {{ field.field_index }}{% if not loop.last %},{% endif %}
    {%- endfor %}
}