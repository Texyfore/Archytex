package routes

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/Texyfore/Archytex/backend/logging"
)

func Hello(w http.ResponseWriter, r *http.Request) {
	var data string
	err := json.NewDecoder(r.Body).Decode(&data)
	if err != nil {
		logging.Error(w, r, err, err.Error(), http.StatusBadRequest)
		return
	}
	num, err := strconv.Atoi(data)
	if err != nil {
		logging.Error(w, r, err, "Could not convert argument to number", http.StatusBadRequest)
		return
	}
	json.NewEncoder(w).Encode(map[string]interface{}{
		"value":   num,
		"squared": num * num,
	})
}
