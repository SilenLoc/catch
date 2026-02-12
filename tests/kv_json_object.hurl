GET {{target}}/kv/test-json-object
HTTP 404

POST {{target}}/kv/test-json-object
{
    "hello": "world"
}
HTTP 200

GET {{target}}/kv/test-json-object
HTTP 200

[Asserts]
body contains "world"
# todo make this run, it should be possible to select this
# the problem is that the response is not json but a json string.
jsonpath "$.hello" == "world"

DELETE {{target}}/kv/test-json-object
HTTP 204

GET {{target}}/kv/test-json-object
HTTP 404