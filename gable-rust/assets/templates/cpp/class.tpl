#pragma once

#include <string>
#include <vector>
{% for import in imports %}#include "{{ import }}.h"
{% endfor %}

/**
 * {{CLASS_NAME}}
 */
class {{CLASS_NAME}} {
public:
    {% for field in info.fields %}
    /**
     * {{field.field_desc}}
     */
    {{ field.field_type }} {{ field.field_name }};
    {% endfor %}
};