#!/bin/bash
# Run this after getting OTP from Gmail:
curl -X POST https://agentwallet.mcpay.tech/api/connect/complete \\
  -H \"Content-Type: application/json\" \\
  -d '{\"username\":\"samitpatel122\",\"email\":\"samitpatel122@gmail.com\",\"otp\":\"YOUR_OTP_HERE\"}' > agentwallet-response.json
cat agentwallet-response.json | jq .apiToken > ~/.agentwallet/config.json.tmp && mv ~/.agentwallet/config.json.tmp ~/.agentwallet/config.json
curl https://agentwallet.mcpay.tech/api/wallets/samitpatel122/actions/faucet-sol -H \"Authorization: Bearer $(cat ~/.agentwallet/config.json)\" -d '{}'