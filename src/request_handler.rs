package request_handler;

import org.json.JSONObject;
import java.util.logging.Logger;

public class RequestHandler {
    private static final Logger logger = Logger.getLogger(RequestHandler.class.getName());

    public void process_query(String rawRequest, StrategyManager strategyManager) {
        clearRawBuffer();
        try {
            JSONObject jsonRequest = new JSONObject(rawRequest);
            String requestType = jsonRequest.getString("type");

            switch (requestType) {
                case "subscribe":
                    handleSubscribe(jsonRequest);
                    break;
                case "apply_strategy":
                    handleApplyStrategy(jsonRequest);
                    break;
                case "stop_strategy":
                    handleStrategyStopCompleted(jsonRequest, strategyManager);
                    break;
                default:
                    logger.warning("Unknown request type: " + requestType);
            }
        } catch (Exception e) {
            logger.severe("Error processing request: " + e.getMessage());
        }
    }

    private void clearRawBuffer() {
        // Implementation to clear the raw buffer
    }

    private void handleSubscribe(JSONObject jsonRequest) {
        // Implementation for handling subscription
    }

    private void handleApplyStrategy(JSONObject jsonRequest) {
        // Implementation for applying strategy
    }

    public void handleStrategyStopCompleted(JSONObject request, StrategyManager strategyManager) {
        String address = request.getString("address_");
        boolean success = strategyManager.stop_strategy(address);
        if (success) {
            logger.info("Successfully stopped strategy for address: " + address);
        } else {
            logger.warning("No strategy found for address: " + address);
        }
    }
}