package models

import "time"

type Render struct {
	Id       interface{} `json:"_id, omitempty" bson:"_id, omitempty"`
	Name     string      `json:"name" bson:"name"`
	Status   float64     `json:"status" bson:"status"`
	Started  time.Time   `json:"started" bson:"started"`
	Finished *time.Time  `json:"finished, omitempty" bson:"finished, omitempty"`
	Icon     string      `json:"icon" bson:"icon"`
}
