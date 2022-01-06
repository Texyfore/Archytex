package models

type Asset struct {
	Id         interface{} `json:"_id, omitempty" bson:"_id, omitempty"`
	InternalId int         `json:"id" bson:"id"`
	Url        string      `json:"url" bson:"url"`
	Public     bool        `json:"public" bson:"public"`
}
