package routes

import (
	"net/http"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/gorilla/websocket"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		//TODO: Restrict origin
		return true
	},
}

func Ws(w http.ResponseWriter, r *http.Request) {
	ws, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		logging.Error(w, r, err, "Could not start WebSocket connection", http.StatusBadRequest)
		return
	}
	defer ws.Close()
	var sess string
	err = ws.ReadJSON(&sess)
	if err != nil {
		logging.ErrorWs(r, ws, err, err.Error(), http.StatusBadRequest)
		return
	}
	oid, err := primitive.ObjectIDFromHex(sess)
	if err != nil {
		logging.ErrorWs(r, ws, err, "Invalid token", http.StatusBadRequest)
		return
	}
	session, err := database.CurrentDatabase.GetSession(oid)
	if err != nil {
		logging.ErrorWs(r, ws, err, "Invalid token", http.StatusBadRequest)
		return
	}
	if session == nil {
		logging.ErrorWs(r, ws, err, "Invalid token", http.StatusBadRequest)
		return
	}

	c, err := database.CurrentDatabase.SubscribeProjects(session.User.Id)
	if err != nil {
		logging.ErrorWs(r, ws, err, "internal server error", http.StatusInternalServerError)
		return
	}
	//TODO: Disconnect
	for v := range c {
		_ = ws.WriteJSON(v)
	}
}
