package logging

import (
	"log"
	"net/http"
)

//TODO: Custom logging

func LogMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		requestId := GenerateRequestId()
		ctx := requestId.Context(r.Context())
		log.Printf("[INFO] %s %s %s", requestId, r.Method, r.URL.Path)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}
