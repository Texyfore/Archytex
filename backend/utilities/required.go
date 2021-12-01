package utilities

import (
	"fmt"
	"reflect"
	"strings"
)

func getJsonTag(t reflect.StructField) string {
	fieldName := t.Name
	jsonTag := t.Tag.Get("json")
	if jsonTag != "" && jsonTag != "-" {
		fieldName = jsonTag
		commaIdx := strings.Index(fieldName, ",")
		if commaIdx > 0 {
			fieldName = fieldName[:commaIdx]
		}
	}
	return fieldName
}

func Required(object interface{}) error {
	v := reflect.ValueOf(object)
	t := v.Type()
	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		if field.Tag.Get("required") != "" {

			if v.Field(i).IsNil() {
				return fmt.Errorf("field '%s' is required", getJsonTag(field))
			}
		}
	}
	return nil
}
