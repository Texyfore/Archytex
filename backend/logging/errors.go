package logging

import (
	"encoding/json"
	"github.com/gorilla/websocket"
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
