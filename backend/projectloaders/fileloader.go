package projectloaders

import (
	"io"
	"math/rand"
	"net/http"
	"os"
	"path"
	"strconv"
)

type FileProjectLoader struct {
	Directory string
}

func fileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func (f FileProjectLoader) projectPath(p string) string {
	return path.Join(f.Directory, p)
}

func (f FileProjectLoader) NewPath() string {
	id := strconv.Itoa(int(rand.Uint64()))
	for fileExists(f.projectPath(id)) {
		id = strconv.Itoa(int(rand.Uint64()))
	}
	return id
}

func (f FileProjectLoader) GetProject(w http.ResponseWriter, r *http.Request, p string) error {
	p = f.projectPath(p)
	if !fileExists(p) {
		w.Write([]byte{})
		return nil
	}
	file, err := os.Open(p)
	if err != nil {
		return err
	}
	_, err = io.Copy(w, file)
	if err != nil {
		return err
	}
	return nil
}
