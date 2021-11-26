package models

import "time"

type Project struct {
	Id      interface{}   `json:"_id, omitempty" bson:"_id, omitempty"`
	Title   string        `json:"title" bson:"title"`
	Created time.Time     `json:"created" bson:"created"`
	Renders []Render      `json:"renders" bson:"renders"`
	Path    string        `json:"path" bson:"path"`
	Assets  []interface{} `json:"assets" bson:"assets"`
}
