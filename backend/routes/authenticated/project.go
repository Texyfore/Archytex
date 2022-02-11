package authenticated

import (
	"encoding/json"
	"net/http"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func Project(w http.ResponseWriter, r *http.Request) {
	session := models.UseSession(r.Context())
	if r.Method == "POST" {
		var name string
		err := json.NewDecoder(r.Body).Decode(&name)
		if err != nil {
			logging.Error(w, r, err, err.Error(), http.StatusBadRequest)
			return
		}
		id, err := database.CurrentDatabase.CreateProject(session.User.Id, name)
		if err != nil {
			logging.Error(w, r, err, "could not create project", http.StatusBadRequest)
			return
		}
		json.NewEncoder(w).Encode(id.(primitive.ObjectID).Hex())
	} else if r.Method == "DELETE" {
		params := mux.Vars(r)
		_projectId, ok := params["id"]
		if !ok {
			logging.Error(w, r, nil, "Project not specified", http.StatusBadRequest)
			return
		}
		projectId, err := primitive.ObjectIDFromHex(_projectId)
		if err != nil {
			logging.Error(w, r, err, "invalid project id", http.StatusBadRequest)
			return
		}
		err = database.CurrentDatabase.DeleteProject(session.User.Id, projectId)
		if err != nil {
			logging.Error(w, r, err, "could not remove project", http.StatusBadRequest)
			return
		}
	} else if r.Method == "PATCH" {
		params := mux.Vars(r)
		_projectId, ok := params["id"]
		if !ok {
			logging.Error(w, r, nil, "Project not specified", http.StatusBadRequest)
			return
		}
		projectId, err := primitive.ObjectIDFromHex(_projectId)
		if err != nil {
			logging.Error(w, r, err, "invalid project id", http.StatusBadRequest)
			return
		}
		var name string
		err = json.NewDecoder(r.Body).Decode(&name)
		if err != nil {
			logging.Error(w, r, err, err.Error(), http.StatusBadRequest)
			return
		}
		err = database.CurrentDatabase.RenameProject(session.User.Id, projectId, name)
		if err != nil {
			logging.Error(w, r, err, "could not rename project", http.StatusBadRequest)
			return
		}
	}
}
