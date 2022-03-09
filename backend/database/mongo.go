package database

import (
	"context"
	"encoding/base64"
	"fmt"
	"math/rand"
	"time"

	"github.com/Texyfore/Archytex/backend/projectloaders"
	"go.mongodb.org/mongo-driver/bson/primitive"

	"github.com/Texyfore/Archytex/backend/database/models"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type MongoDatabase struct {
	ctx       context.Context
	Client    *mongo.Client
	Database  *mongo.Database
	Users     *mongo.Collection
	Registers *mongo.Collection
	Sessions  *mongo.Collection
	Textures  *mongo.Collection
	Props     *mongo.Collection
}

func (m MongoDatabase) GetTextures() ([]models.Asset, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
	defer cancel()
	result, err := m.Textures.Find(ctx, bson.D{})
	if err != nil {
		return nil, err
	}
	if result.Err() != nil {
		return nil, result.Err()
	}
	var assets []models.Asset
	err = result.All(ctx, &assets)
	if err != nil {
		return nil, err
	}
	return assets, nil
}

func (m MongoDatabase) GetTexture(id interface{}) (*models.Asset, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*4)
	defer cancel()
	result := m.Textures.FindOne(ctx, bson.D{{"_id", id}})
	if result.Err() != nil {
		return nil, result.Err()
	}
	var asset models.Asset
	err := result.Decode(&asset)
	if err != nil {
		return nil, err
	}
	return &asset, nil
}

func (m MongoDatabase) GetProps() ([]models.Asset, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
	defer cancel()
	result, err := m.Props.Find(ctx, bson.D{})
	if err != nil {
		return nil, err
	}
	if result.Err() != nil {
		return nil, result.Err()
	}
	var assets []models.Asset
	err = result.All(ctx, &assets)
	if err != nil {
		return nil, err
	}
	return assets, nil
}

func (m MongoDatabase) GetProp(id interface{}) (*models.Asset, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*4)
	defer cancel()
	result := m.Props.FindOne(ctx, bson.D{{"_id", id}})
	if result.Err() != nil {
		return nil, result.Err()
	}
	var asset models.Asset
	err := result.Decode(&asset)
	if err != nil {
		return nil, err
	}
	return &asset, nil
}

func (m MongoDatabase) GetProject(userId interface{}, projectId interface{}) (*models.Project, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer cancel()
	_result := m.Users.FindOne(ctx, bson.D{
		{"_id", userId},
		{"projects._id", projectId},
	}, options.FindOne().SetProjection(bson.D{{"projects.$", 1}}))
	if _result.Err() == mongo.ErrNoDocuments {
		return nil, ErrProjectNotFound
	}
	if _result.Err() != nil {
		return nil, _result.Err()
	}
	var result struct {
		Projects []models.Project `json:"projects"`
	}
	err := _result.Decode(&result)
	if err != nil {
		return nil, err
	}
	return &result.Projects[0], nil
}

func (m MongoDatabase) CreateProject(userId interface{}, name string) (interface{}, error) {
	//TODO implement me
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	project := models.Project{
		Id:      primitive.NewObjectID(),
		Created: time.Now(),
		Renders: []models.Render{},
		Path:    projectloaders.CurrentProjectLoader.NewPath(),
		Title:   name,
	}
	_, err := m.Users.UpdateOne(ctx, bson.D{
		{"_id", userId},
	}, bson.D{
		{"$push", bson.D{
			{"projects", project},
		}},
	})
	return project.Id, err
}

func (m MongoDatabase) CreateRender(userId interface{}, projectId interface{}, name string) (interface{}, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	id := primitive.NewObjectID()
	render := models.Render{
		Id:       id,
		Finished: nil,
		Icon:     "",
		Name:     name,
		Started:  time.Now(),
		Status:   0.0,
	}
	_, err := m.Users.UpdateOne(ctx, bson.D{
		{"_id", userId},
	}, bson.D{
		{"$push", bson.D{
			{"projects.$[elem].renders", render},
		}},
	}, &options.UpdateOptions{
		ArrayFilters: &options.ArrayFilters{
			Filters: []interface{}{bson.M{"elem._id": projectId}},
		},
	})
	return id, err
}

func (m MongoDatabase) RenameProject(userId interface{}, projectId interface{}, name string) error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	res, err := m.Users.UpdateOne(ctx, bson.D{
		{"_id", userId},
		{"projects._id", projectId},
	}, bson.D{
		{"$set", bson.D{
			{"projects.$.title", name},
		}},
	})
	if res.MatchedCount == 0 {
		return ErrProjectNotFound
	}
	return err
}

func (m MongoDatabase) DeleteProject(userId interface{}, projectId interface{}) error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	res, err := m.Users.UpdateOne(ctx, bson.D{
		{"_id", userId},
	}, bson.D{
		{"$pull", bson.D{
			{"projects", bson.D{
				{"_id", projectId},
			}},
		}},
	})
	if res.ModifiedCount == 0 {
		return ErrProjectNotFound
	}
	return err
}

func (m MongoDatabase) DeleteRender(userId interface{}, projectId interface{}, renderId interface{}) error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	res, err := m.Users.UpdateOne(ctx, bson.D{
		{"_id", userId},
		{"projects._id", projectId},
	}, bson.D{
		{"$pull", bson.D{
			{"projects.$.renders", bson.D{
				{"_id", renderId},
			}},
		}},
	})
	if res.ModifiedCount == 0 {
		return ErrProjectNotFound
	}
	return err
}

func (m MongoDatabase) GetSession(id interface{}) (*models.Session, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	cursor, err := m.Sessions.Aggregate(ctx, bson.A{
		bson.D{
			{"$match", bson.D{{"_id", id}}},
		},
		bson.D{
			{"$lookup", bson.D{
				{"from", "users"},
				{"localField", "user_id"},
				{"foreignField", "_id"},
				{"as", "user"},
			}},
		},
		bson.D{
			{"$project", bson.D{
				{"_id", 1},
				{"user", bson.D{
					{"$arrayElemAt", bson.A{"$user", 0}},
				}},
			}},
		},
	})
	if err != nil {
		return nil, err
	}
	var session models.Session
	cursor.Next(ctx)
	err = cursor.Decode(&session)
	if err != nil {
		return nil, err
	}
	return &session, nil
}

func randomToken(n int) (string, error) {
	bytes := make([]byte, n)
	_, err := rand.Read(bytes)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.EncodeToString(bytes), nil
}

func (m MongoDatabase) CreateSession(user *models.User) (string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	id, err := randomToken(128 / 8)
	if err != nil {
		return "", err
	}
	_, err = m.Sessions.InsertOne(ctx, bson.D{
		{"user_id", user.Id},
		{"_id", id},
	})
	if err != nil {
		return "", err
	}
	return id, nil
}

func (m MongoDatabase) GetUserByUsername(username string) (*models.User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*1)
	defer cancel()
	result := m.Users.FindOne(ctx, bson.D{
		{"username", username},
	})
	if result.Err() != nil {
		return nil, result.Err()
	}
	var user models.User
	err := result.Decode(&user)
	if err != nil {
		return nil, err
	}
	return &user, nil
}

func MongoConnect(connectionString string, database string) (*MongoDatabase, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
	defer cancel()
	client, err := mongo.Connect(ctx, options.Client().ApplyURI(connectionString))
	if err != nil {
		return nil, err
	}
	db := client.Database(database)
	usersCollection := db.Collection("users")
	registersCollection := db.Collection("registers")
	sessionsCollection := db.Collection("sessions")
	texturesCollection := db.Collection("textures")
	propsCollection := db.Collection("props")
	return &MongoDatabase{
		Client:    client,
		ctx:       ctx,
		Database:  db,
		Users:     usersCollection,
		Registers: registersCollection,
		Sessions:  sessionsCollection,
		Textures:  texturesCollection,
		Props:     propsCollection,
	}, nil
}

func (m MongoDatabase) Close() {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	m.Client.Disconnect(ctx)
}

func (m MongoDatabase) GetUser(id interface{}) (*models.User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*1)
	defer cancel()
	result := m.Users.FindOne(ctx, bson.D{
		{"_id", id},
	})
	if result.Err() != nil {
		return nil, result.Err()
	}
	var user models.User
	err := result.Decode(&user)
	if err != nil {
		return nil, err
	}
	return &user, nil
}

func (m MongoDatabase) CreateUser(user models.User) (interface{}, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer cancel()
	result, err := m.Users.InsertOne(ctx, user)
	if err != nil {
		return nil, err
	}
	return result.InsertedID, nil
}

func (m MongoDatabase) GetRegister(id interface{}) (*models.Register, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*1)
	defer cancel()
	result := m.Registers.FindOne(ctx, bson.D{
		{"_id", id},
	})
	if result.Err() != nil {
		return nil, result.Err()
	}
	var register models.Register
	err := result.Decode(&register)
	if err != nil {
		return nil, err
	}
	return &register, nil
}
func (m MongoDatabase) GetRegisterByToken(token string) (*models.Register, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*1)
	defer cancel()
	result := m.Registers.FindOne(ctx, bson.D{
		{"token", token},
	})
	if result.Err() != nil {
		return nil, result.Err()
	}
	var register models.Register
	err := result.Decode(&register)
	if err != nil {
		return nil, err
	}
	return &register, nil
}
func (m MongoDatabase) CreateRegister(register models.Register) (interface{}, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer cancel()
	result, err := m.Registers.InsertOne(ctx, register)
	if err != nil {
		return nil, err
	}
	return result.InsertedID, nil
}

func (m MongoDatabase) DeleteRegister(register models.Register) error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer cancel()
	_, err := m.Registers.DeleteOne(ctx, bson.D{{"_id", register.Id}})
	if err != nil {
		return err
	}
	return nil
}

func (m MongoDatabase) UserExists(username, email string) (bool, error) {
	//TODO: find a solution for possible race conditions
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer cancel()
	countUser, err := m.Users.CountDocuments(ctx, bson.D{
		{"$or", bson.A{
			bson.D{{"username", username}},
			bson.D{{"email", email}},
		}},
	})
	if err != nil {
		return false, err
	}
	countRegister, err := m.Registers.CountDocuments(ctx, bson.D{
		{"$or", bson.A{
			bson.D{{"username", username}},
			bson.D{{"email", email}},
		}},
	})
	if err != nil {
		return false, err
	}
	return !(countUser == 0 && countRegister == 0), nil
}

func (m MongoDatabase) SubscribeProjects(userId interface{}) (chan Updates, error) {
	idMatch := bson.D{
		{"$match", bson.D{
			{"fullDocument._id", userId},
		}},
	}
	project := bson.D{
		{"$project", bson.D{
			{"fullDocument.projects._id", 1},
			{"fullDocument.projects.title", 1},
			{"fullDocument.projects.created", 1},
			{"fullDocument.projects.renders._id", 1},
			{"fullDocument.projects.renders.name", 1},
			{"fullDocument.projects.renders.status", 1},
			{"fullDocument.projects.renders.started", 1},
			{"fullDocument.projects.renders.finished", 1},
			{"fullDocument.projects.renders.icon", 1},
		}},
	}
	pipeline := mongo.Pipeline{idMatch, project}
	opts := options.ChangeStream().SetFullDocument(options.UpdateLookup)
	stream, err := m.Users.Watch(context.TODO(), pipeline, opts)
	if err != nil {
		return nil, err
	}
	c := make(chan Updates)
	go func() {
		defer stream.Close(context.TODO())
		defer close(c)
		//Send first update
		r := m.Users.FindOne(context.TODO(), bson.D{
			{"_id", userId},
		})
		var data struct {
			FullDocument Updates `json:"fullDocument" bson:"fullDocument"`
		}
		r.Decode(&data.FullDocument)
		c <- data.FullDocument
		//TODO: Close if user quit
		for stream.Next(context.TODO()) {
			err := stream.Decode(&data)
			if err != nil {
				fmt.Println(err)
				break
			}
			c <- data.FullDocument
		}
	}()
	return c, nil
}
