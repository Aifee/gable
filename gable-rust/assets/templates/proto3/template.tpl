syntax = "proto3";
package Gable;

{%- for item in imports %}
import "{{ item }}.proto";
{%- endfor %}
message {{CLASS_NAME}} { 
    {%- for field in fields %}
    {{ field.field_type }} {{ field.field_name }} = {{ field.field_index }};// {{field.field_desc}} 
    {%- endfor %}
}