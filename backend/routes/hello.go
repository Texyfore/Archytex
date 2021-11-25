package routes

import (
	"encoding/json"
	"net/http"

	"github.com/Texyfore/Archytex/backend/logging"
)

func Hello(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(logging.GetRequestId(r.Context()))
}
