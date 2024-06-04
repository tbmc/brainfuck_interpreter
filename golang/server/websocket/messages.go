package websocket

type MessageToFront struct {
	IsFirstMessage bool   `json:"isFirstMessage"`
	Code           string `json:"code"`
	Input          string `json:"input"`
	Output         string `json:"output"`
	Closed         bool   `json:"closed"`
}

func NewMessage() MessageToFront {
	return MessageToFront{false, "", "", "", false}
}
