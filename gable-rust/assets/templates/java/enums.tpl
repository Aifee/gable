package gable;

/**
 * {{CLASS_NAME}}
 */
public enum {{CLASS_NAME}} {
    {%- for field in info.fields %}
    /**
     * {{field.field_desc}}
     */
    {{ field.field_name }}({{ field.field_index }}){% if not loop.last %},{% else %};{% endif %}
    {%- endfor %}

    private final int value;

    {{CLASS_NAME}}(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }
}