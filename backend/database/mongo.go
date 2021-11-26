package database

import (
	"context"
	"time"

	"github.com/Texyfore/Archytex/backend/database/models"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type MongoDatabase struct {
	cancel   context.CancelFunc
	ctx      context.Context
	Client   *mongo.Client
	Database *mongo.Database
	Users    *mongo.Collection
}

func MongoConnect(connectionString string, database string) (*MongoDatabase, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
	client, err := mongo.Connect(ctx, options.Client().ApplyURI(connectionString))
	if err != nil {
		cancel()
		return nil, err
	}
	db := client.Database(database)
	usersCollection := db.Collection("users")
	return &MongoDatabase{
		cancel:   cancel,
		Client:   client,
		ctx:      ctx,
		Database: db,
		Users:    usersCollection,
	}, nil
}

func (m MongoDatabase) Close() {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	m.Client.Disconnect(ctx)
	m.cancel()
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
