package database

import "github.com/Texyfore/Archytex/backend/database/models"

var CurrentDatabase Database

type Database interface {
	GetUser(id interface{}) (*models.User, error)
	CreateUser(user models.User) (interface{}, error)
}

type UserHandler interface {
	Update() error
	Delete() error
	AddProject(project models.Project) (interface{}, error)
}

type ProjectHandler interface {
	Update() error
	Delete() error
	AddRender(render models.Render) (interface{}, error)
}
