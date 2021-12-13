package models

import "context"

type Session struct {
	Id   interface{} `json:"_id,omitempty" bson:"_id,omitempty"`
	User *User       `json:"user" bson:"user"`
}

type keys string

var SessionKey keys = keys("sessionkey")

func (session Session) WithContext(ctx context.Context) context.Context {
	return context.WithValue(ctx, SessionKey, session)
}

func GetSession(ctx context.Context) *Session {
	session, ok := ctx.Value(SessionKey).(Session)
	if ok {
		return &session
	}
	return nil
}
