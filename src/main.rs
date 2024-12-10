package main

import (
    "log"
    "net"
    "sync"
    "time"
    "github.com/tokio/tokio"
    "github.com/tokio/tokio-net"
)

type Connection struct {
    loginId string
    conn    net.Conn
}

type ConnectionManager struct {
    connections map[string]*Connection
    mu          sync.Mutex
}

func (cm *ConnectionManager) new_connection_requested(loginId string, connection *Connection) {
    cm.mu.Lock()
    cm.connections[loginId] = connection
    cm.mu.Unlock()
}

func (cm *ConnectionManager) connection_closed(loginId string) {
    cm.mu.Lock()
    delete(cm.connections, loginId)
    cm.mu.Unlock()
}

func handle_connection(conn net.Conn, cm *ConnectionManager) {
    defer conn.Close()
    var loginId string
    // Logic to read loginId from conn
    connection := &Connection{loginId: loginId, conn: conn}
    cm.new_connection_requested(loginId, connection)
    defer cm.connection_closed(loginId)

    // Handle incoming data and responses
}

func main() {
    log.SetFlags(log.LstdFlags | log.Lshortfile)
    cm := &ConnectionManager{connections: make(map[string]*Connection)}

    listener, err := net.Listen("tcp", ":8080")
    if err != nil {
        log.Fatalf("Error starting server: %v", err)
    }
    defer listener.Close()

    for {
        conn, err := listener.Accept()
        if err != nil {
            log.Printf("Error accepting connection: %v", err)
            continue
        }
        go func(c net.Conn) {
            tokio::spawn(handle_connection(c, cm))
        }(conn)
    }
}