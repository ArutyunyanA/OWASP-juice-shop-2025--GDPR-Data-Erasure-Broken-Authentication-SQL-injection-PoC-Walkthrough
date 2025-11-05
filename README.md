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

If q is a regular search text, everything is fine. If q opens with the escape character ' or the sequence ), UNION, --, the string literal is closed earlier, and the remainder becomes SQL code. The result is a change in the query structure (injection).
Examples:

```sql
SELECT id, name, description FROM products WHERE name LIKE '%non-existentvalue%'
```

Key elements that change behavior:

Single quote (') — closes the string literal;
Parentheses ()) — adjust syntax depending on the context;
UNION — joins your SELECT with the attacker's SELECT (if the column types are compatible);
- or /* ... */ — comment out the remainder of the original query.

Conclusion: protection must check the feasibility of such transformations to parameterize the counter and control input parts.










