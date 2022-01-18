package authenticated

import (
	"net/http"

	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/projectloaders"
	"github.com/gorilla/mux"
)

func ShowRender(w http.ResponseWriter, r *http.Request) {
	_ = models.UseSession(r.Context())
	params := mux.Vars(r)
	renderId, ok := params["render"]
	if !ok {
		logging.Error(w, r, nil, "Render not specified", http.StatusBadRequest)
		return
	}
	w.Header().Add("Content-Type", "image/png")
	err := projectloaders.CurrentProjectLoader.GetProject(w, r, renderId+".png")
	if err != nil {
		logging.Error(w, r, err, "Could not get render", http.StatusBadRequest)
		return
	}
}
