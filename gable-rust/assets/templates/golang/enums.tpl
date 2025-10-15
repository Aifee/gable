package gable

// {{CLASS_NAME}} 
type {{CLASS_NAME}} int

const (
	{%- for field in info.fields %}
	// {{field.field_desc}}
	{{ field.field_name }} {{CLASS_NAME}} = {{ field.field_index }}
	{%- endfor %}
)

func ({{CLASS_NAME}}) String() string {
	return map[{{CLASS_NAME}}]string{
		{%- for field in info.fields %}
		{{ field.field_name }}: "{{ field.field_name }}",
		{%- endfor %}
	}[{{CLASS_NAME}}]
}

func ({{CLASS_NAME}}) Values() []{{CLASS_NAME}} {
	return []{{CLASS_NAME}}{
		{%- for field in info.fields %}
		{{ field.field_name }},
		{%- endfor %}
	}
}