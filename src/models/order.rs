package models

use std::collections::HashSet;

struct Order {
    unique_id: String,
    price: f64,
    quantity: u32,
}

impl Order {
    fn new(unique_id: String, price: f64, quantity: u32) -> Self {
        Order {
            unique_id,
            price,
            quantity,
        }
    }

    fn modify(&mut self, new_unique_id: String, new_price: f64, new_quantity: u32) {
        self.unique_id = new_unique_id;
        self.price = new_price;
        self.quantity = new_quantity;
    }

    fn get_unique_id(&self) -> &String {
        &self.unique_id
    }

    fn get_price(&self) -> f64 {
        self.price
    }

    fn get_quantity(&self) -> u32 {
        self.quantity
    }

    fn get_attributes(&self) -> (&String, f64, u32) {
        (&self.unique_id, self.price, self.quantity)
    }
}

struct OrderManager {
    orders: HashSet<String>,
}

impl OrderManager {
    fn new() -> Self {
        OrderManager {
            orders: HashSet::new(),
        }
    }

    fn add_order(&mut self, order: Order) -> Result<(), String> {
        if self.orders.contains(&order.unique_id) {
            return Err("Unique ID already exists".to_string());
        }
        self.orders.insert(order.unique_id);
        Ok(())
    }
}