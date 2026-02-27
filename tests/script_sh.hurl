# Testing Inlined Shell File
POST {{target}}/script
Content-Type: application/x-sh
file,script/inline.sh;
HTTP 200

[Asserts]
body == "inline shell test"


# Testing Shell Script - Echo
POST {{target}}/script
Content-Type: application/x-sh
```
echo "Hello from shell"
```
HTTP 200

[Asserts]
body == "Hello from shell"


# Testing Shell Script - Multiple Commands
POST {{target}}/script
Content-Type: application/x-sh
```
NAME="World"
echo "Hello $NAME"
```
HTTP 200

[Asserts]
body == "Hello World"


# Testing Shell Script - Arithmetic
POST {{target}}/script
Content-Type: application/x-sh
```
expr 5 + 3
```
HTTP 200

[Asserts]
body == "8"


# Testing Shell Script - Command Substitution
POST {{target}}/script
Content-Type: application/x-sh
```
echo "Files: $(echo file1 file2 file3)"
```
HTTP 200

[Asserts]
body == "Files: file1 file2 file3"


# Testing Shell Script - Error Case
POST {{target}}/script
Content-Type: application/x-sh
```
exit 1
```
HTTP 400
