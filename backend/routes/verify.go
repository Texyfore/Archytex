package routes

import (
	"net/http"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
)

func Verify(w http.ResponseWriter, r *http.Request) {
	tokens, ok := r.URL.Query()["token"]
	if !ok || len(tokens[0]) < 1 {
		logging.Error(w, r, nil, "invalid token", http.StatusBadRequest)
		return
	}
	token := tokens[0]
	register, err := database.CurrentDatabase.GetRegisterByToken(token)
	if err != nil {
		logging.Error(w, r, err, "invalid token", http.StatusBadRequest)
		return
	}
	err = database.CurrentDatabase.DeleteRegister(*register)
	if err != nil {
		logging.Error(w, r, err, "internal server error", http.StatusInternalServerError)
		return
	}
	user := models.User{
		Username: register.Username,
		Email:    register.Email,
		Password: register.Password,
		Coins:    0,
		Projects: make([]models.Project, 0),
	}
	_, err = database.CurrentDatabase.CreateUser(user)
	if err != nil {
		logging.Error(w, r, err, "internal server error", http.StatusInternalServerError)
		return
	}
	http.Redirect(w, r, "/login", http.StatusSeeOther)
}
