package utils;

import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

public class Logger {
    private static final String LOG_FILE = "application.log";
    private static final DateTimeFormatter formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

    public Logger() {
        initializeLogger();
    }

    private void initializeLogger() {
        try {
            if (!Files.exists(Paths.get(LOG_FILE))) {
                Files.createFile(Paths.get(LOG_FILE));
            }
        } catch (IOException e) {
            logError("Logger initialization error: " + e.getMessage());
        }
    }

    public void log(String message) {
        try (FileWriter fileWriter = new FileWriter(LOG_FILE, true);
             PrintWriter printWriter = new PrintWriter(fileWriter)) {
            String timestamp = LocalDateTime.now().format(formatter);
            printWriter.println(timestamp + " - " + message);
        } catch (IOException e) {
            logError("Logging error: " + e.getMessage());
        }
    }

    public void logRequest(String requestDetails) {
        log("Request: " + requestDetails);
    }

    public void logResponse(String responseDetails) {
        log("Response: " + responseDetails);
    }

    public void logError(String errorMessage) {
        log("Error: " + errorMessage);
    }

    public void processRequest(String requestDetails) {
        logRequest(requestDetails);
        try {
            String responseDetails = "Processed request: " + requestDetails;
            logResponse(responseDetails);
        } catch (Exception e) {
            logError("Processing error: " + e.getMessage());
        }
    }
}