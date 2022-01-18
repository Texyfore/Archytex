package routes

import (
	"encoding/json"
	"net/http"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
)

type assetsResponse struct {
	Props    []models.Asset `json:"props"`
	Textures []models.Asset `json:"textures"`
}

func Assets(w http.ResponseWriter, r *http.Request) {
	props, err := database.CurrentDatabase.GetProps()
	if err != nil {
		logging.Error(w, r, err, "Internal server error", http.StatusInternalServerError)
		return
	}
	textures, err := database.CurrentDatabase.GetTextures()
	if err != nil {
		logging.Error(w, r, err, "Internal server error", http.StatusInternalServerError)
		return
	}
	resp := assetsResponse{
		Props:    props,
		Textures: textures,
	}
	err = json.NewEncoder(w).Encode(&resp)
	if err != nil {
		logging.Error(w, r, err, "Internal server error", http.StatusInternalServerError)
		return
	}
}
