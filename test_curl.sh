#!/bin/bash

# Test script for the Rust web application
# Make sure your server is running before executing these commands

echo "Testing the web application endpoints..."
echo "========================================"

# Test the root endpoint (Hello, world!)
echo "1. Testing GET / (Hello, world! endpoint):"
curl -X GET http://localhost:8000/
echo -e "\n"

# Test with verbose output to see headers
echo "2. Testing GET / with verbose output:"
curl -v -X GET http://localhost:8000/
echo -e "\n"

# Test with different content type headers
echo "3. Testing GET / with Accept header:"
curl -H "Accept: application/json" -X GET http://localhost:8000/
echo -e "\n"

# Test error handling (non-existent endpoint)
echo "4. Testing non-existent endpoint:"
curl -X GET http://localhost:8000/nonexistent
echo -e "\n"

echo "Testing completed!" 