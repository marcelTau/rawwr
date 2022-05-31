
[Current Chapter](https://craftinginterpreters.com/scanning.html#reserved-words-and-identifiers): reserved-words-and-identifiers

12 min 19
refactor error handling


## Terms
### Lexeme
    When lexing source code, you parse lines of ascii letters. For example
    ```lox
    var language = "lox";
    ```
    Grouping them together into meaningful parts like this
    ```lox
    'var' 'language' '=' '"lox"' ';'
    ```
    makes them individual **Lexemes**
    

