package database

import "github.com/Texyfore/Archytex/backend/database/models"

var CurrentDatabase Database

type Database interface {
	GetUser(id interface{}) (*models.User, error)
	GetRegister(id interface{}) (*models.Register, error)
	GetRegisterByToken(token string) (*models.Register, error)
	CreateUser(user models.User) (interface{}, error)
	CreateRegister(register models.Register) (interface{}, error)

	UserExists(username, email string) (bool, error)

	DeleteRegister(register models.Register) error
}
