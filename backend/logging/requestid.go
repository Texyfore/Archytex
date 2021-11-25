package logging

import (
	"context"
	"crypto/rand"
	"encoding/hex"
)

type RequestId string
type keys string

var RequestIdKey keys = keys("requestId")

func GenerateRequestId() RequestId {
	bytes := make([]byte, 8)
	rand.Read(bytes)
	return RequestId(hex.EncodeToString(bytes))
}

func (r RequestId) WithContext(ctx context.Context) context.Context {
	return context.WithValue(ctx, RequestIdKey, r)
}

func GetRequestId(ctx context.Context) RequestId {
	return ctx.Value(RequestIdKey).(RequestId)
}
