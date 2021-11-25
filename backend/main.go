package main

import (
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"time"

	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/routes"
	"github.com/gorilla/mux"
)

func jsonMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Content-Type", "application/json; charset=UTF-8")
		next.ServeHTTP(w, r)
	})
}

func main() {
	rand.Seed(time.Now().UnixNano())

	//TODO: Read port
	port := 8080
	r := mux.NewRouter()
	r.Use(logging.LogMiddleware)
	api := r.PathPrefix("/api").Subrouter()
	api.Use(jsonMiddleware)

	api.HandleFunc("/hello", routes.Hello).Methods("GET")

	http.Handle("/", r)
	fmt.Printf("Listening on port %d\n", port)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", port), r))
}
