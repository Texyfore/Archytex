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
	ctx       context.Context
	Client    *mongo.Client
	Database  *mongo.Database
	Users     *mongo.Collection
	Registers *mongo.Collection
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
	return &MongoDatabase{
		Client:    client,
		ctx:       ctx,
		Database:  db,
		Users:     usersCollection,
		Registers: registersCollection,
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
func (m MongoDatabase) CreateRegister(register models.Register) (interface{}, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer cancel()
	result, err := m.Registers.InsertOne(ctx, register)
	if err != nil {
		return nil, err
	}
	return result.InsertedID, nil
}
