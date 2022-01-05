package projectloaders

import "net/http"

type ProjectLoader interface {
	GetProject(w http.ResponseWriter, r *http.Request, path string) error
	NewPath() string
}

var CurrentProjectLoader ProjectLoader
