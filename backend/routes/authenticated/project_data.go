package authenticated

import (
	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/projectloaders"
	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"net/http"
)

func ProjectData(w http.ResponseWriter, r *http.Request) {
	if r.Method == "GET" {
		params := mux.Vars(r)
		_projectId, ok := params["id"]
		if !ok {
			logging.Error(w, r, nil, "Project not specified", http.StatusBadRequest)
			return
		}
		GetProject(w, r, _projectId)
	}
}

func GetProject(w http.ResponseWriter, r *http.Request, _projectId string) {
	session := models.UseSession(r.Context())
	projectId, err := primitive.ObjectIDFromHex(_projectId)
	if err != nil {
		logging.Error(w, r, err, "invalid project id", http.StatusBadRequest)
		return
	}
	project, err := database.CurrentDatabase.GetProject(session.User.Id, projectId)
	if err == database.ErrProjectNotFound {
		w.WriteHeader(404)
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
