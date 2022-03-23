package session

import (
	"net/http"
	"strings"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/logging"
)

func UserMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		key := r.Header.Get("Authorization")
		if strings.HasPrefix(key, "Bearer ") {
			key = key[len("Bearer "):]
		} else {
			logging.Error(w, r, nil, "missing Bearer token", http.StatusBadRequest)
			return
		}
		session, err := database.CurrentDatabase.GetSession(key)
		if err != nil {
			logging.Error(w, r, err, "unauthorized", http.StatusUnauthorized)
			return
		}
		ctx := session.Context(r.Context())
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}
