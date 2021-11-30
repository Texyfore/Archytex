package routes

import (
	"encoding/json"
	"net/http"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/mailing"
	"github.com/Texyfore/Archytex/backend/utilities"
)

type registerRequest struct {
	Username *string `required:"true" json:"username"`
	Email    *string `required:"true" json:"email"`
	Password *string `required:"true" json:"password"`
	Captcha  *string `required:"true" json:"captcha"`
}

func CheckCaptcha(token string) (bool, error) {
	//TODO: Check captcha
	return token == "GOOD_TOKEN", nil
}

func Register(w http.ResponseWriter, r *http.Request) {
	var data registerRequest
	err := json.NewDecoder(r.Body).Decode(&data)
	if err != nil {
		logging.Error(w, r, err, err.Error(), http.StatusBadRequest)
		return
	}
	err = utilities.Required(data)
	if err != nil {
		logging.Error(w, r, err, err.Error(), http.StatusBadRequest)
		return
	}
	check, err := CheckCaptcha(*data.Captcha)
	if err != nil {
		logging.Error(w, r, err, "Internal server error", http.StatusInternalServerError)
		return
	}
	if !check {
		logging.Error(w, r, nil, "Invalid captcha", http.StatusForbidden)
		return
	}

	register, err := models.NewRegister(*data.Username, *data.Password, *data.Email)
	if err != nil {
		logging.Error(w, r, err, "Failed to register", http.StatusInternalServerError)
		return
	}
	_, err = database.CurrentDatabase.CreateRegister(*register)
	if err != nil {
		logging.Error(w, r, err, "Failed to save account", http.StatusInternalServerError)
		return
	}
	args := make(map[string]string)
	//TODO: Replace with correct URL
	args["Link"] = "http://localhost:8080/api/verify?token=" + register.Token
	args["Username"] = register.Username
	err = mailing.SendTemplate(*data.Email, "Thank you for joining Archytex!", "register", args)
	if err != nil {
		logging.Error(w, r, err, "Failed to send Email", http.StatusInternalServerError)
		return
	}
}
