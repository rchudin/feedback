package validate


// Validator ...
type Validator interface {
	Struct(interface{}) (bool, map[string]string)
}
