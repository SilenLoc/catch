GET {{target}}/kv/key1
HTTP 404

POST {{target}}/kv/key1
{
    "hello": "world"
}
HTTP 200


GET {{target}}/kv/key1
HTTP 200

[Asserts]
body contains "world"
# todo make this run, it should be possible to select this
# the problem is that the response is not json but a json string.
#jsonpath "$.hello" == "world"