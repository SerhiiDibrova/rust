mod models {
    pub mod strategy {
        pub struct StrategyParameters {
            pub param1: i32,
            pub param2: String,
        }

        pub struct Strategy {
            pub id: String,
            pub parameters: StrategyParameters,
        }
    }
}