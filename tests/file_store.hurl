# Test file upload with plain text
POST {{target}}/file/test.txt
`This is a test file content`
HTTP 200

[Asserts]
body contains "File uploaded"


# Test file download
GET {{target}}/file/test.txt
HTTP 200

[Asserts]
body == "This is a test file content"
header "Content-Type" == "text/plain"


# Test file upload with custom path
POST {{target}}/file/document.txt
X-File-Path: /documents/2024
`Important document content`
HTTP 200

[Asserts]
body contains "File uploaded"


# Test file download with custom path
GET {{target}}/file/document.txt
X-File-Path: /documents/2024
HTTP 200

[Asserts]
body == "Important document content"


# Test file upload with nested path
POST {{target}}/file/data.json
X-File-Path: /api/v1/users
`{"name": "test", "email": "test@example.com"}`
HTTP 200

[Asserts]
body contains "File uploaded"


# Test file download with nested path
GET {{target}}/file/data.json
X-File-Path: /api/v1/users
HTTP 200

[Asserts]
body == "{\"name\": \"test\", \"email\": \"test@example.com\"}"
header "Content-Type" == "application/json"


# Test file upload with binary data
POST {{target}}/file/image.bin
X-File-Path: /images
file,files/test.bin;
HTTP 200

[Asserts]
body contains "File uploaded"


# Test file download binary data
GET {{target}}/file/image.bin
X-File-Path: /images
HTTP 200

[Asserts]
header "Content-Type" == "application/octet-stream"


# Test file delete
DELETE {{target}}/file/test.txt
HTTP 204


# Test file not found after delete
GET {{target}}/file/test.txt
HTTP 404

[Asserts]
body == "File not found"


# Test file delete with custom path
DELETE {{target}}/file/document.txt
X-File-Path: /documents/2024
HTTP 204


# Test file not found with custom path
GET {{target}}/file/document.txt
X-File-Path: /documents/2024
HTTP 404
