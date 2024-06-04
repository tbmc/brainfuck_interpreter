package websocket

import (
	"encoding/json"
	"fmt"
	"github.com/gorilla/websocket"
	"log"
	"net/http"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  10_240,
	WriteBufferSize: 10_240,
}

func writer(conn *websocket.Conn, output chan []byte) {
	defer func() {
		conn.Close()
	}()

	for {
		messageBytes := <-output
		var data MessageToFront

		if messageBytes == nil {
			data = MessageToFront{false, "", "", "", true}
		} else {
			message := string(messageBytes)
			data = MessageToFront{false, "", "", message, false}
		}

		err := conn.WriteJSON(data)
		if err != nil {
			return
		}
	}
}

func executeClientForSocket(conn *websocket.Conn) {
	defer func() {
		conn.Close()
	}()

	isFirst := true
	input := make(chan []byte, 1_000)
	output := make(chan []byte, 1_000)

	close := func() {
		conn.Close()
	}

	for {
		_, message, err := conn.ReadMessage()

		if err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				log.Printf("error: %v\n", err)
			}
			break
		}

		if isFirst {
			isFirst = false
			var data MessageToFront
			err := json.Unmarshal(message, &data)
			if err != nil {
				return
			}
			if !data.IsFirstMessage {
				_ = fmt.Errorf("the message is not first message")
			}
			go writer(conn, output)
			go execute(data.Code, input, output, close)
		} else {
			data := NewMessage()
			err := json.Unmarshal(message, &data)
			if err != nil {
				return
			}
			input <- []byte(data.Input)
		}
	}
}

func ServeClient(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println(err)
	}

	go executeClientForSocket(conn)
}
