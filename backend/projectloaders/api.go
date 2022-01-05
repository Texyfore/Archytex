package projectloaders

import (
	"io"
	"net/http"
)

type ProjectLoader interface {
	GetProject(w http.ResponseWriter, r *http.Request, path string) error
	SaveProject(r io.Reader, path string) error
	NewPath() string
}

var CurrentProjectLoader ProjectLoader
