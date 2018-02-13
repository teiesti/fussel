# Fussel Lint Index

## `file_encoding`

### Example

```
warning: file encoding should be consistent
 --> src/main.rs:9:11
```


## `incomplete_work`

### Example

```plain
warning: there should be no incomplete work
 --> src/main.rs:9:3
  |
9 | // TODO
  |    ^^^^ keyword suggests incomplete work
  |
  = help: remove keyword after completing work
  = note: 'TODO', 'FIXME' and 'DEBUG' are recognized as keywords for incomplete work
```


## `indentation`

Indentation should be consistent.

### Example

```
warning: tabs should be avoided
 --> src/main.rs:9:0
  |
9 |     let n = 42;
  | ^^^^ tab found here
  |
  = help: use spaces instead of tabs
```

```
warning: indentation should be a multiple of 4
 --> src/main.rs:9:0
  |
9 |    let n = 42;
  | ^^^ indentation is not a multiple of 4
  |
```


## `line_break`

### Example

```
warning: line breaks should be consistent
 --> src/main.rs:9:11
  |
9 | let n = 42;
  |            ^^ CRLF found here
  |
  = help: use LF instead
```


## `line_length`

### example

```
warning: line length should not exceed 20 characters
 --> src/main.rs:9:101
  |
9 | let text = "This is long!";
  |                      ^^^^^^ limit exceeded here
  |
```


## `trailing_newline`

### Example

```
warning: files should end with a trailing newline
 --> src/main.rs:9:11
  |
9 | let n = 42;
  |            ^ trailing newline missing here
  |
```


## `trailing_whitespace`

### Example

```
warning: lines should not end with trailing whitespace, unless the file format requires
 --> src/main.rs:9:11
  |
9 | let n = 42;
  |            ^^^ trailing whitespace found here
  |
  = note: filenames ending with '.md' are ignored
```
