# {{CLASS_NAME}}.py

{%- for import in imports %}
from {{ import }} import {{ import }}
{%- endfor %}

class {{CLASS_NAME}}:
    """
    {{CLASS_NAME}}
    """
    def __init__(self):
        {%- for field in info.fields %}
        # {{field.field_desc}}
        self.{{ field.field_name }} = None
        {%- endfor %}