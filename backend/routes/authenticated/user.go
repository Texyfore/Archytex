package authenticated

import (
	"encoding/json"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"net/http"
)

type userResponse struct {
	Username string  `json:"username"`
	Email    string  `json:"email"`
	Coins    float64 `json:"coins"`
}

func User(w http.ResponseWriter, r *http.Request) {
	session := models.UseSession(r.Context())
	if session == nil {
		logging.Error(w, r, nil, "unauthorized", http.StatusUnauthorized)
		return
	}
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

}
