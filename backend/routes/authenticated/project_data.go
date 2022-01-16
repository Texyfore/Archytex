package authenticated

import (
	"net/http"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/projectloaders"
	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func ProjectData(w http.ResponseWriter, r *http.Request) {
	params := mux.Vars(r)
	_projectId, ok := params["id"]
	if !ok {
		logging.Error(w, r, nil, "Project not specified", http.StatusBadRequest)
		return
	}
	if r.Method == "GET" {
		GetProject(w, r, _projectId)
	} else if r.Method == "POST" {
		SaveProject(w, r, _projectId)
	}
}

func loadProject(r *http.Request, _projectId string) (*models.Project, error) {
	session := models.UseSession(r.Context())
	projectId, err := primitive.ObjectIDFromHex(_projectId)
	if err != nil {
		return nil, err
	}
	project, err := database.CurrentDatabase.GetProject(session.User.Id, projectId)

	if err != nil {
		return nil, err
	}
	return project, nil
}

func SaveProject(w http.ResponseWriter, r *http.Request, _projectId string) {
	project, err := loadProject(r, _projectId)
	if err == database.ErrProjectNotFound {
		logging.Error(w, r, err, "Project not found", http.StatusNotFound)
		return
	}
	if err != nil {
		logging.Error(w, r, err, "Project Unavailable", http.StatusInternalServerError)
		return
	}
	err = projectloaders.CurrentProjectLoader.SaveProject(r.Body, project.Path)
	if err != nil {
		logging.Error(w, r, err, "Unable to save project", http.StatusInternalServerError)
		return
	}

}

func GetProject(w http.ResponseWriter, r *http.Request, _projectId string) {
	project, err := loadProject(r, _projectId)
	if err == database.ErrProjectNotFound {
		logging.Error(w, r, err, "Project not found", http.StatusNotFound)
		return
	}
	if err != nil {
		logging.Error(w, r, err, "Project Unavailable", http.StatusInternalServerError)
		return
	}
	err = projectloaders.CurrentProjectLoader.GetProject(w, r, project.Path)
	if err != nil {
		logging.Error(w, r, err, "Project Unavailable", http.StatusInternalServerError)
		return
	}
}
