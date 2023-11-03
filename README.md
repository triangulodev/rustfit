# Rustfit

Fitness studio management SaaS.

## Technical Decisions

_Based on [ideas by Yegor Bugayenko](https://www.yegor256.com/2015/08/04/nine-steps-start-software-project.html)_

### Decisions and Alternatives

Rust is the programming language.
This decision is based on my wish to learn and explore the language. We can
also justify using Rust because of the small binary and closer to the metal
code it compiles to. Rust's type system is also a great tool.

Axum is the web application framework.
As it is just a thin layer on top of tokio, our async runtime for Rust.

sqlite is the database.
Because of its simplicity and ease of deployment.

sqlx is the SQL toolkit of choice.
We've decided not to use an ORM to keep the project flexible and simple using
SQL directly.

### Concerns

### Assumptions

### Risks

## Project

> A story is a simple narrative that describes an end user in some domain-level role going through some domain-level process to achieve a domain-level, and valuable, outcome. It does not describe a computer program. — https://holub.com/reading/

- **Managing Staff** [MS]
- **Managing Users** [MU]
- **Classes** [C]
- **Billing Users** [BU]
- **Billing Accounts** [BA]
- **Manage My Studio** [MMS]

---

_Initial flow to setup an account on Rustfit._

[MS] As an owner, manage my account.

- Create an account with login credentials, so I can have access to my account.
  - Frequency: Once
  - Value: Critical
  - Technical details: Set account access to user. Set user as admin role.
- Login account with login credentials.

---

[MS] As an owner, add staff, be sure staff can't change important details about the account. Staff should have their own access credentials.

- Implement sessions so accounts can continue logged in
- Implement Organization, which roles will be assigned to
- Create role for an account. There can only be a single account role per Organization.

---

[MU] As an owner, invite users. Users are people that I provide a fitness service to.

- Create an invite token which will be assigned to the user

[MU] As a user, create account when invited.

---

[C] As an owner, manage locations.

---

[C] As an owner, create classes. Classes have a name, schedule, teacher (User), location, size.

---

[C] As a user, join a class. Classes should be joined daily.

---

[C] As staff or above, remove users from class.

---
