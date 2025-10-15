syntax = "proto2";
package Gable;

{%- for item in imports %}
import "{{ item }}.proto";
{%- endfor %}
message {{CLASS_NAME}} { 
    {%- for field in info.fields %}
    {%- if field.field_type is starting_with("repeated") %}{{ field.field_type }} {{ field.field_name }} = {{ field.field_index }};{% else %}optional {{ field.field_type }} {{ field.field_name }} = {{ field.field_index }}{{ field.field_extend }};{% endif %}// {{field.field_desc}} 
    {%- endfor %}
}