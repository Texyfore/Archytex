package routes

import (
	"encoding/json"
	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/logging"
	"net/http"
)

type loginRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type loginResponse struct {
	Token string
}

func Login(w http.ResponseWriter, r *http.Request) {
	var data loginRequest
	err := json.NewDecoder(r.Body).Decode(&data)
	if err != nil {
		logging.Error(w, r, err, err.Error(), http.StatusBadRequest)
		return
	}
	user, err := database.CurrentDatabase.GetUserByUsername(data.Username)
	if err != nil {
		logging.Error(w, r, err, "Invalid username or password", http.StatusBadRequest)
		return
	}
	check, err := user.CheckPassword(data.Password)
	if err != nil || !check {
		logging.Error(w, r, err, "Invalid username or password", http.StatusBadRequest)
		return
	}
	id, err := database.CurrentDatabase.CreateSession(user)
	if err != nil {
		logging.Error(w, r, err, "Internal server error", http.StatusInternalServerError)
		return
	}
	resp := loginResponse{Token: id}
	err = json.NewEncoder(w).Encode(resp)
	if err != nil {
		logging.Error(w, r, err, "internal server error", http.StatusInternalServerError)
		return
	}
}
