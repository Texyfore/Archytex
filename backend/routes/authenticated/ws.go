package authenticated

import (
	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/gorilla/websocket"
	"net/http"
)

var upgrader = websocket.Upgrader{}

func Ws(w http.ResponseWriter, r *http.Request) {
	session := models.UseSession(r.Context())
	ws, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		logging.Error(w, r, err, "Could not start WebSocket connection", http.StatusBadRequest)
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
