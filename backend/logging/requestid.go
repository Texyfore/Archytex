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

func (r RequestId) Context(ctx context.Context) context.Context {
	return context.WithValue(ctx, RequestIdKey, r)
}

func UseRequestId(ctx context.Context) RequestId {
	return ctx.Value(RequestIdKey).(RequestId)
}
