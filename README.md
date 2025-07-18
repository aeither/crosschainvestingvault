# Cross Chain Vesting Vault

A Rust web application built with Axum.

## Testing

### Quick Test
To test the application with curl:

```bash
# Basic test
curl http://localhost:8000/

# Expected output: "Hello, world!"
```

### Comprehensive Testing
Run the test script for a complete set of tests:

```bash
./test_curl.sh
```

### Manual Testing Commands

```bash
# Test the root endpoint
curl -X GET http://localhost:8000/

# Test with verbose output
curl -v -X GET http://localhost:8000/

# Test with custom headers
curl -H "Accept: application/json" -X GET http://localhost:8000/

# Test error handling
curl -X GET http://localhost:8000/nonexistent
```

## Running the Application

Make sure to start your server before running the tests. The application should be running on `localhost:8000`.
