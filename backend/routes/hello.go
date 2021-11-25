package routes

import (
	"encoding/json"
	"net/http"
)

func Hello(w http.ResponseWriter, r *http.Request) {

	json.NewEncoder(w).Encode("Hello World!")
}
