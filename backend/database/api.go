package database

import (
	"errors"
	"github.com/Texyfore/Archytex/backend/database/models"
	"time"
)

var CurrentDatabase Database

type Database interface {
	GetUser(id interface{}) (*models.User, error)
	GetUserByUsername(username string) (*models.User, error)
	GetRegister(id interface{}) (*models.Register, error)
	GetRegisterByToken(token string) (*models.Register, error)
	CreateUser(user models.User) (interface{}, error)
	CreateRegister(register models.Register) (interface{}, error)

	UserExists(username, email string) (bool, error)

	DeleteRegister(register models.Register) error

	GetSession(id interface{}) (*models.Session, error)
	CreateSession(user *models.User) (string, error)

	CreateProject(userId interface{}, name string) error
	RenameProject(userId interface{}, projectId interface{}, name string) error
	DeleteProject(userId interface{}, projectId interface{}) error
	GetProject(userId interface{}, projectId interface{}) (*models.Project, error)
	SubscribeProjects(userId interface{}) (chan Updates, error)
}

var ErrProjectNotFound = errors.New("project not found")

type Updates struct {
	Projects []ProjectUpdate `json:"projects" bson:"projects"`
}

type ProjectUpdate struct {
	Id      string         `json:"id" bson:"_id"`
	Title   string         `json:"title" bson:"title"`
	Created time.Time      `json:"created" bson:"created"`
	Renders []RenderUpdate `json:"renders" bson:"renders"`
}

type RenderUpdate struct {
	Id       string     `json:"id" bson:"_id"`
	Name     string     `json:"name" bson:"name"`
	Status   float64    `json:"status" bson:"status"`
	Started  time.Time  `json:"started" bson:"started"`
	Finished *time.Time `json:"finished, omitempty" bson:"finished, omitempty"`
	Icon     string     `json:"icon" bson:"icon"`
}
