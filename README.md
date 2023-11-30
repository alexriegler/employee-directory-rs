# Employee Directory

This a project inspired by an exercise listed in [*The Rust Programming Language*](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary).

Excerpt from chapter 8 section 3:

```
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
```

Commands:

```
Add <employee> to <department>
Remove <employee> from <department>
Print (department <department>|company)
```

Example run:

```
> ./employee-directory.exe
Welcome to the Employee Directory!
> Add Sally to Engineering
Successfully added Sally to Engineering.
> Add Amir to Sales
Successfully added Amir to Sales.
> Print department Engineering
Department Engineering:
Sally
> Print company
Department Engineering:
Sally
Department Sales:
Amir
> quit
```
