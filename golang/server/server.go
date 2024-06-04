package server

import (
	"brainfuck/server/websocket"
	"fmt"
	"net/http"
	"net/http/httputil"
	"net/url"
)

func Serve(port int) {
	remote, err := url.Parse("http://localhost:3000")
	if err != nil {
		panic(err.Error())
	}

	proxy := httputil.NewSingleHostReverseProxy(remote)

	http.HandleFunc("/api/ws", func(w http.ResponseWriter, r *http.Request) {
		websocket.ServeClient(w, r)
	})

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		proxy.ServeHTTP(w, r)
	})

	err = http.ListenAndServe(fmt.Sprintf("0.0.0.0:%d", port), nil)
	if err != nil {
		panic(err.Error())
	}

}
