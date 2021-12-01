package logging

import (
	"encoding/json"
	"log"
	"net/http"
)

type errorMessage struct {
	Message   string    `json:"message"`
	RequestId RequestId `json:"_requestId"`
}

func Error(w http.ResponseWriter, r *http.Request, err error, message string, code int) {
	errLogged := ""
	if err == nil {
		errLogged = message
	} else {
		errLogged = err.Error()
	}
	w.WriteHeader(code)
	requestId := GetRequestId(r.Context())
	log.Printf("[ERROR] %s %s", requestId, errLogged)
	json.NewEncoder(w).Encode(errorMessage{
		Message:   message,
		RequestId: requestId,
	})
}
