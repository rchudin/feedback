package validator

import (
	"feedback/internal/constants"
	"feedback/internal/validate"
	"reflect"
	"strings"

	"github.com/go-playground/validator/v10"
)

// Validator ...
type Validator struct {
	validate *validator.Validate
}

var _ validate.Validator = (*Validator)(nil)

// NewValidator ...
func NewValidator() *Validator {
	newValidator := validator.New()

	newValidator.RegisterTagNameFunc(func(fld reflect.StructField) string {
		name := strings.SplitN(fld.Tag.Get("json"), ",", 2)[0]
		if name == "-" {
			return ""
		}
		return name
	})

	return &Validator{
		validate: newValidator,
	}
}

// Struct ...
func (v *Validator) Struct(data interface{}) (bool, map[string]string) {

	// Проверка структуры
	// Structure check
	err := v.validate.Struct(data)

	// Если данные некорректны
	// If the data is incorrect
	if err != nil {
		errorsMap := make(map[string]string)

		// Поиск внутренних ошибок валидатора
		// Search for internal validator errors
		if err, ok := err.(*validator.InvalidValidationError); ok {
			errorsMap["validator"] = err.Error()
			return false, errorsMap
		}

		// Структуризация ошибок
		// Error structuring
		for _, e := range err.(validator.ValidationErrors) {

			var name string
			if name = e.Field(); name == "" {
				name = strings.ToLower(e.StructField())
			}

			switch e.Tag() {
			case "required":
				errorsMap[name] = "the " + name + " is required"
				break
			case "email":
				//errors[name] = append(errors[name], "must be a valid email address")
				errorsMap[name] = constants.ErrMustBeAValidEmailAddress.Error()
				break
			default:
				errorsMap[name] = "the " + name + " is invalid"
				break
			}
		}

		return true, errorsMap
	}
	return true, nil
}
