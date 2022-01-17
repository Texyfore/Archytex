package authenticated

import (
	"context"
	"fmt"
	"io/ioutil"
	"net/http"
	"time"

	"github.com/Texyfore/Archytex/backend/database"
	"github.com/Texyfore/Archytex/backend/database/models"
	"github.com/Texyfore/Archytex/backend/logging"
	"github.com/gorilla/mux"
	"github.com/streadway/amqp"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func Render(w http.ResponseWriter, r *http.Request) {
	session := models.UseSession(r.Context())
	params := mux.Vars(r)
	_projectId, ok := params["id"]
	if !ok {
		logging.Error(w, r, nil, "Project not specified", http.StatusBadRequest)
		return
	}
	projectId, err := primitive.ObjectIDFromHex(_projectId)
	if err != nil {
		logging.Error(w, r, err, "invalid project id", http.StatusBadRequest)
		return
	}
	id, err := database.CurrentDatabase.CreateRender(session.User.Id, projectId, "PLACEHOLDER")
	if err != nil {
		logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
		return
	}
	task_id := id.(primitive.ObjectID).Hex()
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	//TODO: Get width, height and sample count from frontend
	bytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		logging.Error(w, r, err, "couldn't create render", http.StatusBadGateway)
		return
	}
	err = database.RedisClient.Set(ctx, fmt.Sprintf("archyrt:%s:width", task_id), 512, 0).Err()
	if err != nil {
		logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
		return
	}
	err = database.RedisClient.Set(ctx, fmt.Sprintf("archyrt:%s:height", task_id), 512, 0).Err()
	if err != nil {
		logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
		return
	}
	err = database.RedisClient.Set(ctx, fmt.Sprintf("archyrt:%s:samples", task_id), 1, 0).Err()
	if err != nil {
		logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
		return
	}
	err = database.RedisClient.Set(ctx, fmt.Sprintf("archyrt:%s:scene", task_id), bytes, 0).Err()
	if err != nil {
		logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
		return
	}
	err = database.RabbitmqChannel.Publish("", "archyrt:dispatch", false, false, amqp.Publishing{
		ContentType: "text/plain",
		Body:        []byte(task_id),
	})
	if err != nil {
		logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
		return
	}
}
