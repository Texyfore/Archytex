package main

import (
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"os"
	"strconv"
	"time"

	"github.com/Texyfore/Archytex/backend/projectloaders"
	"github.com/Texyfore/Archytex/backend/routes/authenticated"
	"github.com/Texyfore/Archytex/backend/session"
	"github.com/go-redis/redis/v8"
	"github.com/streadway/amqp"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/Texyfore/Archytex/backend/routes"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"

	_ "github.com/joho/godotenv/autoload"
)

func jsonMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Content-Type", "application/json; charset=UTF-8")
		next.ServeHTTP(w, r)
	})
}

func main() {
	rand.Seed(time.Now().UnixNano())

	db, err := database.MongoConnect(os.Getenv("MONGO_URI"), os.Getenv("MONGO_DB"))
	if err != nil {
		panic(err)
	}

	amqp, err := amqp.Dial(os.Getenv("AMQP_ADDR"))
	if err != nil {
		panic(err)
	}
	channel, err := amqp.Channel()
	if err != nil {
		panic(err)
	}
	database.RabbitmqChannel = channel

	opt, err := redis.ParseURL(os.Getenv("REDIS_ADDR"))
	if err != nil {
		panic(err)
	}
	database.RedisClient = redis.NewClient(opt)
	if err != nil {
		panic(err)
	}

	database.CurrentDatabase = db
	projectloaders.CurrentProjectLoader = projectloaders.FileProjectLoader{Directory: os.Getenv("PROJECTS_PATH")}

	port, err := strconv.Atoi(os.Getenv("PORT"))
	if err != nil {
		panic(err)
	}
	r := mux.NewRouter()
	r.Use(logging.LogMiddleware)
	api := r.PathPrefix("/api").Subrouter()
	api.Use(jsonMiddleware)

	api.HandleFunc("/hello", routes.Hello).Methods("GET")
	api.HandleFunc("/register", routes.Register).Methods("POST")
	api.HandleFunc("/login", routes.Login).Methods("POST")
	api.HandleFunc("/verify", routes.Verify).Methods("GET")
	api.HandleFunc("/assets", routes.Assets).Methods("GET")
	api.HandleFunc("/ws", routes.Ws)
	api.HandleFunc("/render/{render}", authenticated.ShowRender).Methods("GET")

	auth := api.PathPrefix("/auth").Subrouter()
	auth.Use(session.UserMiddleware)
	auth.HandleFunc("/hello", routes.Hello).Methods("GET")
	auth.HandleFunc("/user", authenticated.User).Methods("POST")
	auth.HandleFunc("/project", authenticated.Project).Methods("POST")
	auth.HandleFunc("/project/{id}", authenticated.Project).Methods("DELETE", "PATCH")
	auth.HandleFunc("/project/{id}/data", authenticated.ProjectData).Methods("GET", "POST")
	auth.HandleFunc("/project/{id}/render/{width}/{height}/{samples}", authenticated.Render).Methods("POST")
	auth.HandleFunc("/project/{id}/render/{render}", authenticated.Render).Methods("DELETE")

	http.Handle("/", r)
	fmt.Printf("Listening on port %d\n", port)

	//TODO: Restrict access origin
	headersOk := handlers.AllowedHeaders([]string{"X-Requested-With", "Content-Type", "Authorization"})
	originsOk := handlers.AllowedOrigins([]string{"*"})
	methodsOk := handlers.AllowedMethods([]string{"GET", "HEAD", "POST", "PUT", "OPTIONS", "DELETE", "PATCH"})

	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", port), handlers.CORS(headersOk, originsOk, methodsOk)(r)))
}
