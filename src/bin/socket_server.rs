package main

import (
    "fmt"
    "log"
    "net"
    "os"
    "sync"
)

type SocketServer struct {
    port        string
    listener    net.Listener
    connections sync.WaitGroup
}

func NewSocketServer(port string) *SocketServer {
    return &SocketServer{port: port}
}

func (s *SocketServer) startAccept() {
    var err error
    s.listener, err = net.Listen("tcp", ":"+s.port)
    if err != nil {
        log.Fatalf("Error starting server: %v", err)
    }
    defer s.listener.Close()

    log.Printf("Server listening on port %s", s.port)

    for {
        conn, err := s.listener.Accept()
        if err != nil {
            log.Printf("Error accepting connection: %v", err)
            continue
        }
        s.connections.Add(1)
        log.Printf("Accepted connection from %s", conn.RemoteAddr().String())
        go s.handleConnection(conn)
    }
}

func (s *SocketServer) handleConnection(conn net.Conn) {
    defer s.connections.Done()
    defer func() {
        if err := conn.Close(); err != nil {
            log.Printf("Error closing connection from %s: %v", conn.RemoteAddr().String(), err)
        }
        log.Printf("Connection from %s closed", conn.RemoteAddr().String())
    }()
    // Handle connection logic here
}

func (s *SocketServer) runServer() {
    s.startAccept()
    s.connections.Wait()
}

func main() {
    if len(os.Args) < 2 {
        log.Fatalf("Usage: %s <port>", os.Args[0])
    }
    port := os.Args[1]
    server := NewSocketServer(port)
    server.runServer()
}