# Markdown Specification

## Block
- \*Hello\* => *Hello* \<i>\</i>
- \*\*Hello\*\* => **Hello** \<strong>\</strong>
- \`let x = 10\` => `let x = 10;` \<code>\</code>


## Inline
---

### Headline

\### Hello
#### Hello

---

### Bullet List

\- x  
\- y  
\- z  

- x
- y
- z
  - z1
  - z2

---

### Ordered List

1\. x  
2\. y  
3\. z

1. x
2. y
3. z

---

### Code Block

\``` rust  
let x = 100;  
let y = String::from("z");  
\```

``` rust  
let x = 100;  
let y = String::from("z");  
```

---

### BlockQuote

\>Hello!   
\>Good Bye!

>Hello!  
>Good Bye!

---

### Table

\| 左寄せ  | 中央寄せ   | 右寄せ   |  
\| :----- | :------: | -----: |  
\| a      |    b     |      c |  
\| d      |    e     |      f |

| 左寄せ  | 中央寄せ   | 右寄せ   |
| :----- | :------: | -----: |
| a      |    b     |      c |
| d      |    e     |      f |

---

### Container

::: xxx

Ok

:::

---

## Reference
https://gihyo.jp/article/2022/08/gihyojp-markdown#gh9ILAwmxz