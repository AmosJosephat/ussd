# API Usage Examples

## HTTP REST API

### Base URL
```
http://localhost:8080
```

### 1. Health Check

**Request**:
```bash
curl http://localhost:8080/health
```

**Response**:
```
200 OK
```

### 2. Start USSD Session

**Request**:
```bash
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "sess_123456789",
    "phone_number": "+1234567890",
    "input": "",
    "service_code": "*123#"
  }'
```

**Response**:
```json
{
  "message": "Welcome to USSD Service\n1. Check Balance\n2. Transfer Money\n3. Help\n",
  "continue_session": true,
  "session_id": "sess_123456789"
}
```

### 3. Select Menu Option (Check Balance)

**Request**:
```bash
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "sess_123456789",
    "phone_number": "+1234567890",
    "input": "1",
    "service_code": "*123#"
  }'
```

**Response**:
```json
{
  "message": "Your balance is: $0.00",
  "continue_session": false,
  "session_id": "sess_123456789"
}
```

### 4. Transfer Money Flow

#### Step 1: Select Transfer Option
**Request**:
```bash
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "sess_987654321",
    "phone_number": "+1234567890",
    "input": "2",
    "service_code": "*123#"
  }'
```

**Response**:
```json
{
  "message": "Enter recipient phone number:",
  "continue_session": true,
  "session_id": "sess_987654321"
}
```

#### Step 2: Enter Recipient
**Request**:
```bash
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "sess_987654321",
    "phone_number": "+1234567890",
    "input": "+9876543210",
    "service_code": "*123#"
  }'
```

**Response**:
```json
{
  "message": "Enter amount:",
  "continue_session": true,
  "session_id": "sess_987654321"
}
```

#### Step 3: Enter Amount
**Request**:
```bash
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "sess_987654321",
    "phone_number": "+1234567890",
    "input": "100.50",
    "service_code": "*123#"
  }'
```

**Response**:
```json
{
  "message": "Transfer of $100.50 to +9876543210 is being processed",
  "continue_session": false,
  "session_id": "sess_987654321"
}
```

### 5. Get Session Details

**Request**:
```bash
curl http://localhost:8080/api/v1/session/sess_123456789
```

**Response**:
```json
{
  "id": "sess_123456789",
  "phone_number": "+1234567890",
  "service_code": "*123#",
  "current_state": "BALANCE",
  "context": {
    "menu_START_selection": "1"
  },
  "created_at": "2025-01-15T10:00:00Z",
  "updated_at": "2025-01-15T10:00:30Z",
  "language": "en",
  "user_id": null,
  "metadata": {}
}
```

### 6. Metrics Endpoint

**Request**:
```bash
curl http://localhost:8080/metrics
```

**Response** (Prometheus format):
```
# HELP ussd_requests_total Total number of USSD requests
# TYPE ussd_requests_total counter
ussd_requests_total{method="POST",endpoint="/api/v1/ussd",status="200"} 42

# HELP ussd_sessions_total Total number of USSD sessions created
# TYPE ussd_sessions_total counter
ussd_sessions_total{service_code="*123#"} 15

# HELP ussd_active_sessions Number of currently active sessions
# TYPE ussd_active_sessions gauge
ussd_active_sessions 3

# HELP ussd_request_duration_seconds Request latency in seconds
# TYPE ussd_request_duration_seconds histogram
ussd_request_duration_seconds_bucket{method="POST",endpoint="/api/v1/ussd",le="0.005"} 20
ussd_request_duration_seconds_bucket{method="POST",endpoint="/api/v1/ussd",le="0.01"} 35
...
```

## Error Responses

### Invalid Input

**Request**:
```bash
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "sess_123",
    "phone_number": "+1234567890",
    "input": "99",
    "service_code": "*123#"
  }'
```

**Response**:
```json
{
  "error": {
    "code": "INVALID_INPUT",
    "message": "Invalid option: 99",
    "status": 400
  }
}
```

### Session Not Found

**Request**:
```bash
curl http://localhost:8080/api/v1/session/nonexistent
```

**Response**:
```json
{
  "error": {
    "code": "SESSION_NOT_FOUND",
    "message": "Session not found: nonexistent",
    "status": 404
  }
}
```

### Rate Limit Exceeded

**Response**:
```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded",
    "status": 429
  }
}
```

## Language Support

Change session language by including it in the session context:

**Request**:
```bash
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "sess_new",
    "phone_number": "+1234567890",
    "input": "",
    "service_code": "*123#"
  }'
```

For Spanish responses, the menus would automatically display in Spanish if the user's language preference is set.

## Testing with curl

### Complete Session Flow

```bash
# Create session and get main menu
SESSION_ID="test_$(date +%s)"

echo "Step 1: Starting session..."
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d "{
    \"session_id\": \"$SESSION_ID\",
    \"phone_number\": \"+1234567890\",
    \"input\": \"\",
    \"service_code\": \"*123#\"
  }" | jq

sleep 1

echo -e "\nStep 2: Selecting option 1 (Balance)..."
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d "{
    \"session_id\": \"$SESSION_ID\",
    \"phone_number\": \"+1234567890\",
    \"input\": \"1\",
    \"service_code\": \"*123#\"
  }" | jq
```

### Load Testing with Apache Bench

```bash
# Install apache2-utils first
# Ubuntu: sudo apt-get install apache2-utils
# Mac: brew install ab

# Test 1000 requests with 10 concurrent connections
ab -n 1000 -c 10 -p request.json -T 'application/json' \
  http://localhost:8080/api/v1/ussd
```

**request.json**:
```json
{
  "session_id": "load_test_001",
  "phone_number": "+1234567890",
  "input": "",
  "service_code": "*123#"
}
```

## Integration Examples

### Python

```python
import requests

class UssdClient:
    def __init__(self, base_url="http://localhost:8080"):
        self.base_url = base_url

    def send_request(self, session_id, phone_number, input_text, service_code="*123#"):
        response = requests.post(
            f"{self.base_url}/api/v1/ussd",
            json={
                "session_id": session_id,
                "phone_number": phone_number,
                "input": input_text,
                "service_code": service_code
            }
        )
        return response.json()

# Usage
client = UssdClient()
response = client.send_request("test_123", "+1234567890", "")
print(response["message"])
```

### JavaScript/Node.js

```javascript
const axios = require('axios');

class UssdClient {
    constructor(baseUrl = 'http://localhost:8080') {
        this.baseUrl = baseUrl;
    }

    async sendRequest(sessionId, phoneNumber, input, serviceCode = '*123#') {
        const response = await axios.post(`${this.baseUrl}/api/v1/ussd`, {
            session_id: sessionId,
            phone_number: phoneNumber,
            input: input,
            service_code: serviceCode
        });
        return response.data;
    }
}

// Usage
const client = new UssdClient();
client.sendRequest('test_123', '+1234567890', '')
    .then(response => console.log(response.message));
```

### Go

```go
package main

import (
    "bytes"
    "encoding/json"
    "net/http"
)

type UssdRequest struct {
    SessionID   string `json:"session_id"`
    PhoneNumber string `json:"phone_number"`
    Input       string `json:"input"`
    ServiceCode string `json:"service_code"`
}

type UssdResponse struct {
    Message         string `json:"message"`
    ContinueSession bool   `json:"continue_session"`
    SessionID       string `json:"session_id"`
}

func sendUssdRequest(sessionID, phone, input string) (*UssdResponse, error) {
    req := UssdRequest{
        SessionID:   sessionID,
        PhoneNumber: phone,
        Input:       input,
        ServiceCode: "*123#",
    }

    jsonData, _ := json.Marshal(req)
    resp, err := http.Post(
        "http://localhost:8080/api/v1/ussd",
        "application/json",
        bytes.NewBuffer(jsonData),
    )
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()

    var response UssdResponse
    json.NewDecoder(resp.Body).Decode(&response)
    return &response, nil
}
```

---

For more examples and documentation, visit the project repository.
