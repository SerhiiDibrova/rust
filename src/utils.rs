package utils;

import org.json.JSONObject;
import java.util.Base64;
import java.util.logging.Level;
import java.util.logging.Logger;

public class Utility {

    private static final Logger logger = Logger.getLogger(Utility.class.getName());

    public static void log_message(String message) {
        logger.log(Level.INFO, message);
    }

    public static void log_warning(String warningMessage) {
        logger.log(Level.WARNING, warningMessage);
    }

    public static void log_error(String errorMessage) {
        logger.log(Level.SEVERE, errorMessage);
    }

    public static void validate_request(JSONObject json) throws IllegalArgumentException {
        if (!json.has("requiredField") || !(json.get("requiredField") instanceof String)) {
            throw new IllegalArgumentException("Invalid request: requiredField is missing or not a string.");
        }
    }

    public static String decrypt(String encryptedMessage) {
        byte[] decryptedBytes = Base64.getDecoder().decode(encryptedMessage);
        return new String(decryptedBytes);
    }

    public static String decrypt_message(String encryptedMessage) {
        return decrypt(encryptedMessage);
    }

    public static String encrypt(String response) {
        return Base64.getEncoder().encodeToString(response.getBytes());
    }

    public static String encrypt_status(String status) {
        return encrypt(status);
    }

    public static JSONObject parse_json(String jsonString) {
        return new JSONObject(jsonString);
    }

    public static String demangle_user_id(String userId) {
        return userId.replaceAll("[^a-zA-Z0-9]", "");
    }

    public static String demangle_strategy_id(String strategyId) {
        return strategyId.replaceAll("[^a-zA-Z0-9]", "");
    }

    public static void closeConnection(AutoCloseable connection) {
        try {
            if (connection != null) {
                connection.close();
            }
        } catch (Exception e) {
            log_error("Error closing connection: " + e.getMessage());
        }
    }

    public static String load_strategy(String name, String parameters) {
        return "Strategy loaded: " + name;
    }

    public static String get_current_status(String strategyId) {
        return "Current status for strategy " + strategyId;
    }
}