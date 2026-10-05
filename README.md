# calcrepl

A simple statement parsing loop. Expressions are implemented using Pratt parsing. Main goals of the project were learning this algorithm and peeking at rust lifetimes.
My main learning resource was [this article](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).

## EBNF ([wikipedia](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form))

```
script = { let_stmt | info_stmt | exit_stmt | expr_stmt } EOF ;

expr_stmt = expression ";" ;
let_stmt = "let" "=" expression ";" ;
info_stmt = "info" ";" ;
exit_stmt = "exit" ";" ;

expression = assignment ;
assignment = ( identifier "=" assignment ) | term ;
term = factor { ( "+" | "-" ) factor } ;
factor = unary { ( "*" | "/" ) unary } ;
unary =  "!" ( unary | primary ) ;
primary = number | identifier | "(" expression ")" ;
```

Expressions are just your avarage arithmetical expressions with regular operator precedence.

## Roadmap
- [x] statements
- [x] global variables
- [ ] statement parsing loop (parse more than one line at once)
- [ ] error recovery (not panic on every error)