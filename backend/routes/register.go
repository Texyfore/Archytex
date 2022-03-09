package routes

import (
	"encoding/json"
	"net/http"
	"os"
	"time"

	"github.com/Texyfore/Archytex/backend/mailing"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/utilities"
	"gopkg.in/ezzarghili/recaptcha-go.v4"
)

type registerRequest struct {
	Username *string `required:"true" json:"username"`
	Email    *string `required:"true" json:"email"`
	Password *string `required:"true" json:"password"`
	Captcha  *string `required:"true" json:"captcha"`
}

func CheckCaptcha(r *http.Request, token string) (bool, error) {
	captcha, err := recaptcha.NewReCAPTCHA(os.Getenv("CAPTCHA_SECRET"), recaptcha.V2, time.Second*3)
	if err != nil {
		return false, err
	}
	err = captcha.Verify(token)
	return err == nil, err
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
	check, err := CheckCaptcha(r, *data.Captcha)
	if err != nil {
		logging.Error(w, r, err, "Invalid captcha", http.StatusInternalServerError)
		return
	}
	if !check {
		logging.Error(w, r, nil, "Invalid captcha", http.StatusForbidden)
		return
	}
	exists, err := database.CurrentDatabase.UserExists(*data.Username, *data.Email)
	if err != nil {
		logging.Error(w, r, err, "internal server error", http.StatusInternalServerError)
		return
	}
	if exists {
		logging.Error(w, r, nil, "user already exists", http.StatusBadRequest)
		return
	}

	register, err := models.NewRegister(*data.Username, *data.Password, *data.Email)
	if err != nil {
		logging.Error(w, r, err, "Failed to register", http.StatusInternalServerError)
		return
	}

	args := make(map[string]string)
	args["Link"] = os.Getenv("DOMAIN") + "/api/verify?token=" + register.Token
	args["Username"] = register.Username
	err = mailing.SendTemplate(*data.Email, "Thank you for joining Archytex!", "register", args)
	if err != nil {
		logging.Error(w, r, err, "Failed to send Email", http.StatusInternalServerError)
		return
	}

	_, err = database.CurrentDatabase.CreateRegister(*register)
	if err != nil {
		logging.Error(w, r, err, "Failed to save account", http.StatusInternalServerError)
		return
	}
}
