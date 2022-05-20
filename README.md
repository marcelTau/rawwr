
[Current Chapter](https://craftinginterpreters.com/scanning.html#location-information): Location Information 


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
    

