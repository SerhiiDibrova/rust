package main

import (
    "encoding/json"
    "fmt"
    "io"
    "log"
    "net"
    "sync"
    "golang.org/x/crypto/nacl/secretbox"
    "golang.org/x/crypto/nacl/box"
    "time"
)

type Server struct {
    listener net.Listener
    pool     *sync.Pool
}

func NewServer(address string) *Server {
    listener, err := net.Listen("tcp", address)
    if err != nil {
        log.Fatalf("Failed to start server: %v", err)
    }
    return &Server{
        listener: listener,
        pool:     &sync.Pool{},
    }
}

func (s *Server) Start() {
    defer s.listener.Close()
    log.Println("Server started on", s.listener.Addr())
    for {
        conn, err := s.listener.Accept()
        if err != nil {
            log.Printf("Failed to accept connection: %v", err)
            continue
        }
        go s.handleConnection(conn)
    }
}

func (s *Server) handleConnection(conn net.Conn) {
    defer conn.Close()
    var buf [512]byte
    n, err := conn.Read(buf[:])
    if err != nil {
        log.Printf("Failed to read from connection: %v", err)
        return
    }
    request := string(buf[:n])
    s.routeRequest(request, conn)
}

func (s *Server) routeRequest(request string, conn net.Conn) {
    switch {
    case request == "GET /strategy-status":
        s.getStrategyStatus(conn)
    case request == "POST /subscribe":
        s.subscribe(conn)
    case request == "POST /new-order":
        s.newOrder(conn)
    case request == "POST /modify-order":
        s.modifyOrder(conn)
    case request == "POST /register-stock-packet":
        s.registerStockPacket(conn)
    default:
        http.Error(conn, "Not Found", http.StatusNotFound)
    }
}

func (s *Server) getStrategyStatus(conn net.Conn) {
    status := map[string]string{"status": "active"}
    response, err := json.Marshal(status)
    if err != nil {
        log.Printf("Failed to marshal JSON: %v", err)
        http.Error(conn, "Internal Server Error", http.StatusInternalServerError)
        return
    }
    conn.Write(response)
}

func (s *Server) subscribe(conn net.Conn) {
    conn.Write([]byte("Subscribed"))
}

func (s *Server) newOrder(conn net.Conn) {
    conn.Write([]byte("New order received"))
}

func (s *Server) modifyOrder(conn net.Conn) {
    conn.Write([]byte("Order modified"))
}

func (s *Server) registerStockPacket(conn net.Conn) {
    conn.Write([]byte("Stock packet registered"))
}

func main() {
    server := NewServer(":8080")
    log.Println("Server started on :8080")
    server.Start()
}