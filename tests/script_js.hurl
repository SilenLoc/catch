# Testing Inlined Files
POST {{target}}/script
file,script/inline.js;
HTTP 200

[Asserts]
body == "inline test"


# Testing Integers
POST {{target}}/script
```
	const add = (a, b) => a + b;
	add(10, 20)
```
HTTP 200

[Asserts]
body == "30"


# Testing Floats
POST {{target}}/script
```
	const add = (a, b) => a + b;
	add(1.5, 1.25)
```
HTTP 200

[Asserts]
body == "2.75"


# Testing Strings
POST {{target}}/script
```
	let a = ['foo', 'bar'];
	a.reduce((prev, curr) => `${prev} ${curr}`);
```
HTTP 200

[Asserts]
body == "foo bar"


# Testing Boolean
POST {{target}}/script
```
	let a = true;
	let b = false;
	a || b
```
HTTP 200

[Asserts]
body == "true"


# Testing Arrays
POST {{target}}/script
```
	const arr = Array.from({ length: 10 }, (_, i) => i + 1);
	JSON.stringify(arr)
```
HTTP 200

[Asserts]
body == "[1,2,3,4,5,6,7,8,9,10]"


# Testing Objects
POST {{target}}/script
```
	let last_name = 'Muster';
	let user = {
		name: last_name,
		date_of_birth: '1900-01-01'
	}
	JSON.stringify(user)
```
HTTP 200

[Asserts]
body == "{\"name\":\"Muster\",\"date_of_birth\":\"1900-01-01\"}"


# Testing Undefined
POST {{target}}/script
```
	let x = 1;
	// not returning anything
```
HTTP 200

[Asserts]
body == "undefined"


# Testing Null
POST {{target}}/script
```
	let x = null;
	x
```
HTTP 200

[Asserts]
body == "null"


# Testing Input Errors
# TODO: Currently the crate "ion" is panicking when an invalid script is run instead of bubbling the error
#POST {{target}}/script
#```
#	let x = x;
#	x
#```
#HTTP 400
