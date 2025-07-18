
Running the Project
Deploy Shuttle Backend:

bash
shuttle deploy
Build ink! Contract:

bash
cargo contract build
Run DRink! Tests:

bash
cargo test
Test API Endpoints:

bash
curl -X POST http://localhost:8000/xcm/claim \
  -H "Content-Type: application/json" \
  -d '{"user_account":"Alice","amount":1000,"destination_parachain":"AssetHub"}'
This implementation hits all the major hackathon tracks and demonstrates a production-ready cross-chain vesting vault with proper security, testing, and deployment automation! 