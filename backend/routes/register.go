package routes

import (
	"encoding/json"
	"net/http"

	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/utilities"
)

type registerRequest struct {
	Username *string `required:"true" json:"username"`
	Email    *string `required:"true" json:"email"`
	Password *string `required:"true" json:"password"`
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

}
