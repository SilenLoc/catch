GET {{target}}/kv/test-plaintext-number
HTTP 404

POST {{target}}/kv/test-plaintext-number
`123`
HTTP 200

GET {{target}}/kv/test-plaintext-number
HTTP 200

[Asserts]
header "Content-Type" contains "text/plain"
body == "123"


DELETE {{target}}/kv/test-plaintext-number
HTTP 204

GET {{target}}/kv/test-plaintext-number
HTTP 404