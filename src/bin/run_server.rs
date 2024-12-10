package main

import (
    "log"
    "net"
    "os"
    "github.com/bitly/go-simplejson"
)

type Config struct {
    Port string `json:"port"`
}

type SocketServer struct {
    port string
}

func (s *SocketServer) StartAccept() {
    listener, err := net.Listen("tcp", ":"+s.port)
    if err != nil {
        log.Fatalf("Error starting server: %v", err)
    }
    defer listener.Close()
    log.Printf("Server started on port %s", s.port)

    for {
        conn, err := listener.Accept()
        if err != nil {
            log.Printf("Error accepting connection: %v", err)
            continue
        }
        go handleConnection(conn)
    }
}

func handleConnection(conn net.Conn) {
    defer conn.Close()
}

func loadConfig(filename string) (Config, error) {
    file, err := os.Open(filename)
    if err != nil {
        return Config{}, err
    }
    defer file.Close()

    jsonData, err := simplejson.NewFromReader(file)
    if err != nil {
        return Config{}, err
    }

    port, err := jsonData.Get("port").String()
    if err != nil {
        return Config{}, err
    }

    return Config{Port: port}, nil
}

func main() {
    config, err := loadConfig("merlin.json")
    if err != nil {
        log.Fatalf("Error loading configuration from merlin.json: %v", err)
    }

    server := SocketServer{port: config.Port}
    server.StartAccept()
}