package models

import "github.com/matthewhartstonge/argon2"

type User struct {
	Id       interface{} `json:"_id,omitempty" bson:"_id,omitempty"`
	Username string      `json:"username" bson:"username"`
	Email    string      `json:"email" bson:"email"`
	Password string      `json:"-" bson:"password"`
	Coins    float64     `json:"coins" bson:"coins"`
	Projects []Project   `json:"projects" bson:"projects"`
}

func (user User) CheckPassword(password string) (bool, error) {
	return argon2.VerifyEncoded([]byte(password), []byte(user.Password))
}
