package authenticated

import (
	"encoding/json"
	"net/http"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
)

type userResponse struct {
	Username string  `json:"username"`
	Email    string  `json:"email"`
	Coins    float64 `json:"coins"`
}

type userRequest struct {
	Username *string `json:"username"`
	Password *string `json:"password"`
	Email    *string `json:"email"`
}

func User(w http.ResponseWriter, r *http.Request) {
	session := models.UseSession(r.Context())
	if session == nil {
		logging.Error(w, r, nil, "unauthorized", http.StatusUnauthorized)
		return
	}
	if r.Method == "POST" {
		resp := userResponse{
			Username: session.User.Username,
			Email:    session.User.Email,
			Coins:    session.User.Coins,
		}
		err := json.NewEncoder(w).Encode(&resp)
		if err != nil {
			logging.Error(w, r, err, "Internal server error", http.StatusInternalServerError)
			return
		}
	} else if r.Method == "PATCH" {
		var req userRequest
		err := json.NewDecoder(r.Body).Decode(&req)
		if err != nil {
			logging.Error(w, r, err, err.Error(), http.StatusInternalServerError)
			return
		}
		err = database.CurrentDatabase.UserModify(session.User.Id, req.Username, req.Email, req.Password)
		if err != nil {
			logging.Error(w, r, err, err.Error(), http.StatusInternalServerError)
			return
		}
	}

}
