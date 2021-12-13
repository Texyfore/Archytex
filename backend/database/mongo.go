package database

import (
	"context"
	"errors"
	"go.mongodb.org/mongo-driver/bson/primitive"
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
	Sessions  *mongo.Collection
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

func (m MongoDatabase) CreateSession(user *models.User) (string, error) {
	//TODO: Use a more secure token
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	result, err := m.Sessions.InsertOne(ctx, bson.D{
		{"user_id", user.Id},
	})
	if err != nil {
		return "", err
	}
	id, ok := result.InsertedID.(primitive.ObjectID)
	if !ok {
		return "", errors.New("id type was not ObjectID")
	}
	return id.Hex(), nil
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
	return &MongoDatabase{
		Client:    client,
		ctx:       ctx,
		Database:  db,
		Users:     usersCollection,
		Registers: registersCollection,
		Sessions:  sessionsCollection,
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
