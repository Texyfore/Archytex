package authenticated

import (
	"context"
	"fmt"
	"io/ioutil"
	"net/http"
	"strconv"
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
	if r.Method == "DELETE" {
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
		_renderId, ok := params["render"]
		if !ok {
			logging.Error(w, r, nil, "Render not specified", http.StatusBadRequest)
			return
		}
		renderId, err := primitive.ObjectIDFromHex(_renderId)
		if err != nil {
			logging.Error(w, r, err, "invalid render id", http.StatusBadRequest)
			return
		}
		err = database.CurrentDatabase.DeleteRender(session.User.Id, projectId, renderId)
		if err == database.ErrProjectNotFound {
			logging.Error(w, r, err, "Project or Render not found", http.StatusNotFound)
			return
		}
		if err != nil {
			logging.Error(w, r, err, "could not remove render", http.StatusBadRequest)
			return
		}
	} else if r.Method == "POST" {
		_projectId, ok := params["id"]
		if !ok {
			logging.Error(w, r, nil, "Project not specified", http.StatusBadRequest)
			return
		}
		_width, _ := params["width"]
		width, err := strconv.Atoi(_width)
		if err != nil {
			logging.Error(w, r, err, "invalid width field", http.StatusBadRequest)
			return
		}
		_height, _ := params["height"]
		height, err := strconv.Atoi(_height)
		if err != nil {
			logging.Error(w, r, err, "invalid height field", http.StatusBadRequest)
			return
		}
		_samples, _ := params["samples"]
		samples, err := strconv.Atoi(_samples)
		if err != nil {
			logging.Error(w, r, err, "invalid samples field", http.StatusBadRequest)
			return
		}
		if width%4 != 0 || height%4 != 0 {
			logging.Error(w, r, err, "Width and Height have to be divisible by 4", http.StatusBadRequest)
			return
		}
		projectId, err := primitive.ObjectIDFromHex(_projectId)
		if err != nil {
			logging.Error(w, r, err, "invalid project id", http.StatusBadRequest)
			return
		}
		project, err := database.CurrentDatabase.GetProject(session.User.Id, projectId)
		if err != nil || project == nil {
			logging.Error(w, r, err, "could not find project", http.StatusNotFound)
			return
		}
		name := fmt.Sprintf("%s-%d", project.Title, len(project.Renders)+1)
		id, err := database.CurrentDatabase.CreateRender(session.User.Id, projectId, name)
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
		err = database.RedisClient.Set(ctx, fmt.Sprintf("archyrt:%s:width", task_id), width, 0).Err()
		if err != nil {
			logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
			return
		}
		err = database.RedisClient.Set(ctx, fmt.Sprintf("archyrt:%s:height", task_id), height, 0).Err()
		if err != nil {
			logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
			return
		}
		err = database.RedisClient.Set(ctx, fmt.Sprintf("archyrt:%s:samples", task_id), samples, 0).Err()
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
			Body:        []byte(task_id + "#" + session.User.Id.(primitive.ObjectID).Hex() + "#" + projectId.Hex()),
		})
		if err != nil {
			logging.Error(w, r, err, "couldn't create render", http.StatusInternalServerError)
			return
		}
	}
}
