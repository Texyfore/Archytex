package logging

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

type errorMessage struct {
	Message   string    `json:"error"`
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
	requestId := UseRequestId(r.Context())
	log.Printf("[ERROR] %s %s", requestId, errLogged)
	json.NewEncoder(w).Encode(errorMessage{
		Message:   message,
		RequestId: requestId,
	})
}

func ErrorWs(r *http.Request, ws *websocket.Conn, err error, message string, code int) {
	errLogged := ""
	if err == nil {
		errLogged = message
	} else {
		errLogged = err.Error()
	}
	requestId := UseRequestId(r.Context())
	log.Printf("[ERROR] %s %s", requestId, errLogged)
	ws.WriteJSON(errorMessage{
		Message:   message,
		RequestId: requestId,
	})
}
