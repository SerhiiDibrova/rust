package main

import (
    "log"
    "sync"
    "time"
)

type StockPacket struct {
    ID     string
    Status string
}

type ResponseType int

const (
    Success ResponseType = iota
    Failure
)

type ConnectionManager struct {
    connections map[string]bool
    mu          sync.RWMutex
}

func (cm *ConnectionManager) Exists(connectionID string) bool {
    cm.mu.RLock()
    defer cm.mu.RUnlock()
    _, exists := cm.connections[connectionID]
    return exists
}

func (cm *ConnectionManager) InitializeConnection(connectionID string) {
    cm.mu.Lock()
    defer cm.mu.Unlock()
    cm.connections[connectionID] = true
}

type MessageQueue struct {
    messages []StockPacket
    mu       sync.Mutex
}

func (mq *MessageQueue) Pop() *StockPacket {
    mq.mu.Lock()
    defer mq.mu.Unlock()
    if len(mq.messages) == 0 {
        return nil
    }
    msg := mq.messages[0]
    mq.messages = mq.messages[1:]
    return &msg
}

func (mq *MessageQueue) Push(msg StockPacket) {
    mq.mu.Lock()
    defer mq.mu.Unlock()
    mq.messages = append(mq.messages, msg)
}

type ArthurMessageManagerThread struct {
    messageQueue      *MessageQueue
    connectionManager *ConnectionManager
}

func (amm *ArthurMessageManagerThread) demangleConnectionID(msg *StockPacket) string {
    return msg.ID // Replace with actual demangling logic based on msg
}

func (amm *ArthurMessageManagerThread) processMessage(msg *StockPacket) ResponseType {
    connectionID := amm.demangleConnectionID(msg)
    if !amm.connectionManager.Exists(connectionID) {
        log.Printf("Connection ID %s does not exist", connectionID)
        return Failure
    }
    log.Printf("Processing StockPacket ID: %s with Status: %s", msg.ID, msg.Status)
    return Success
}

func (amm *ArthurMessageManagerThread) run() {
    for {
        msg := amm.messageQueue.Pop()
        if msg != nil {
            response := amm.processMessage(msg)
            if response == Failure {
                log.Printf("Failed to process message ID: %s", msg.ID)
            } else {
                log.Printf("Processed message with response: %v", response)
            }
        }
        time.Sleep(10 * time.Millisecond)
    }
}

func main() {
    messageQueue := &MessageQueue{messages: []StockPacket{}}
    connectionManager := &ConnectionManager{connections: make(map[string]bool)}
    connectionManager.InitializeConnection("demangled_connection_id") 
    amm := &ArthurMessageManagerThread{
        messageQueue:      messageQueue,
        connectionManager: connectionManager,
    }
    go amm.run()

    for i := 0; i < 10; i++ {
        messageQueue.Push(StockPacket{ID: "msg" + string(i), Status: "new"})
        time.Sleep(50 * time.Millisecond)
    }

    select {}
}