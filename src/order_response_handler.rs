package order_response_handler

import (
    "encoding/json"
    "log"
    "net/http"
    "regexp"
    "strconv"
    "time"
)

type OrderResponse struct {
    OrderId     string  `json:"orderId"`
    StockSymbol string  `json:"stockSymbol"`
    Quantity    int     `json:"quantity"`
    Price       float64 `json:"price"`
    Timestamp   string  `json:"timestamp"`
    Status      string  `json:"status"`
}

var validStatuses = map[string]struct{}{
    "Completed": {},
    "Canceled":  {},
    "Failed":    {},
}

func handle_order_response(w http.ResponseWriter, r *http.Request) {
    if r.Method != http.MethodPost {
        http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
        return
    }

    var orderResponse OrderResponse
    if err := json.NewDecoder(r.Body).Decode(&orderResponse); err != nil {
        log.Println("Error: Invalid request body", err)
        http.Error(w, "Invalid request body", http.StatusBadRequest)
        return
    }

    if orderResponse.OrderId == "" {
        log.Println("Error: orderId is empty")
        http.Error(w, "orderId cannot be empty", http.StatusBadRequest)
        return
    }

    if matched, _ := regexp.MatchString("^[A-Z]{1,5}$", orderResponse.StockSymbol); !matched {
        log.Println("Error: stockSymbol is invalid")
        http.Error(w, "stockSymbol must be 1 to 5 uppercase letters", http.StatusBadRequest)
        return
    }

    if orderResponse.Quantity <= 0 {
        log.Println("Error: quantity must be a positive integer")
        http.Error(w, "quantity must be a positive integer", http.StatusBadRequest)
        return
    }

    if orderResponse.Price < 0 {
        log.Println("Error: price must be a non-negative float")
        http.Error(w, "price must be a non-negative float", http.StatusBadRequest)
        return
    }

    if _, err := time.Parse(time.RFC3339, orderResponse.Timestamp); err != nil {
        log.Println("Error: timestamp is invalid")
        http.Error(w, "timestamp must be a valid datetime format", http.StatusBadRequest)
        return
    }

    if _, valid := validStatuses[orderResponse.Status]; !valid {
        log.Println("Error: status is invalid")
        http.Error(w, "status must be one of the defined values", http.StatusBadRequest)
        return
    }

    log.Printf("Order details: OrderId=%s, StockSymbol=%s, Quantity=%d, Price=%.2f, Timestamp=%s, Status=%s\n",
        orderResponse.OrderId, orderResponse.StockSymbol, orderResponse.Quantity, orderResponse.Price, orderResponse.Timestamp, orderResponse.Status)

    switch orderResponse.Status {
    case "Completed":
        markOrderAsCompleted(orderResponse.OrderId)
        log.Printf("Order %s marked as completed", orderResponse.OrderId)
    case "Canceled":
        markOrderAsCanceled(orderResponse.OrderId)
        log.Printf("Order %s marked as canceled", orderResponse.OrderId)
    case "Failed":
        markOrderAsFailed(orderResponse.OrderId)
        log.Printf("Order %s marked as failed", orderResponse.OrderId)
        handleError(orderResponse.OrderId)
    }

    if err := notifyComponents(orderResponse); err != nil {
        log.Println("Error notifying components:", err)
        http.Error(w, "Failed to notify components", http.StatusInternalServerError)
        return
    }

    w.WriteHeader(http.StatusOK)
}

func markOrderAsCompleted(orderId string) {
}

func markOrderAsCanceled(orderId string) {
}

func markOrderAsFailed(orderId string) {
}

func handleError(orderId string) {
}

func notifyComponents(orderResponse OrderResponse) error {
    return nil
}