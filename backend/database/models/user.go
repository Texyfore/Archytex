package models

type User struct {
	Id       interface{} `json:"_id,omitempty" bson:"_id,omitempty"`
	Username string      `json:"username" bson:"username"`
	Email    string      `json:"email" bson:"email"`
	Password string      `json:"password" bson:"password"`
	Coins    float64     `json:"coins" bson:"coins"`
	Projects []Project   `json:"projects" bson:"projects"`
}
