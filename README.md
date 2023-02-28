## **About** 
A parser that gets an equation and produces an equation but with more context!

---

## **Example**

If you parse a string ```x=y^2+5```, the parser will output the same but every character in the equation gets more context (Token with meaning) that can be processed further to create a DSL but for Mathematical Expression.

######  <br>  

## Here is the Debug Formatted Output:

```rust
[Token { text: "x", kind: Identifier }, Token { text: "=", kind: Sign }, Token { text: "y", kind: Identifier }, Token { text: "^", kind: Cap }, Token { text: "2", kind: Int }, Token { text: "+", kind: Sign }, Token { text: "5", kind: Int }]
```
---

## **Why I made this?**
I am always curious about how our high level code is compiled down to the ultimate machine code that gets our job done with us sitting at the higher level of abstraction. I tried to break into the black box with this project. It gave me a good mental model about how a series of characters get more context through each process such as lexer and parsing and semantic analysis (to check the correctness of the program syntax), we get the machine code that our CPU actually operates on.

---