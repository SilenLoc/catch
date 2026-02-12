GET {{target}}/kv/test-plaintext-string
HTTP 404

POST {{target}}/kv/test-plaintext-string
`Hello, world!`
HTTP 200

GET {{target}}/kv/test-plaintext-string
HTTP 200

[Asserts]
header "Content-Type" contains "text/plain"
body == "Hello, world!"

DELETE {{target}}/kv/test-plaintext-string
HTTP 204

GET {{target}}/kv/test-plaintext-string
HTTP 404