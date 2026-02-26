# Test that adds data to the key-value store without cleanup
# This allows inspection of the UI with data present

# Add a simple string
POST {{target}}/kv/greeting
X-Context: demo-namespace
`Hello from Catch!`
HTTP 200

# Add a JSON object
POST {{target}}/kv/user
X-Context: demo-namespace
{
    "name": "Alice",
    "email": "alice@example.com",
    "role": "admin"
}
HTTP 200

# Add a larger JSON object with nested data
POST {{target}}/kv/config
X-Context: demo-namespace
{
    "server": {
        "host": "localhost",
        "port": 8080,
        "ssl": true
    },
    "database": {
        "url": "postgresql://localhost/mydb",
        "max_connections": 10
    },
    "features": {
        "authentication": true,
        "logging": true,
        "metrics": false
    }
}
HTTP 200

# Add a number
POST {{target}}/kv/counter
X-Context: demo-namespace
`42`
HTTP 200

# Add a JSON array
POST {{target}}/kv/tags
X-Context: demo-namespace
["rust", "web", "actix", "htmx"]
HTTP 200

# Verify all values were stored correctly
GET {{target}}/kv/greeting
X-Context: demo-namespace
HTTP 200
[Asserts]
body == "Hello from Catch!"

GET {{target}}/kv/user
X-Context: demo-namespace
HTTP 200
[Asserts]
body contains "Alice"

GET {{target}}/kv/config
X-Context: demo-namespace
HTTP 200
[Asserts]
body contains "postgresql"

GET {{target}}/kv/counter
X-Context: demo-namespace
HTTP 200
[Asserts]
body == "42"

GET {{target}}/kv/tags
X-Context: demo-namespace
HTTP 200
[Asserts]
body contains "rust"
