syntax = "proto3";
package Gable;

enum {{CLASS_NAME}} { 
    {%- for field in info.fields %}
    {{ field.field_name }} = {{ field.field_index }};// {{field.field_desc}} 
    {%- endfor %}
}