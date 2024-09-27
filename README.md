# columnrs

A command-line tool for formatting tabular data into columns table. Just like `column` terminal command in linux, but can align right.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)

## Installation

You can install columnrs using Cargo, the Rust package manager. Run the following command:

```
$ cargo install columnrs
```

## Usage

```
$ echo "no name age\n1 Tom 4" | columnrs
no  name  age
 1   Tom    4
```

默认数字右对齐：

```
$ echo "no name age\n1 Tom 4\n2 Jerry 15" | columnrs
no   name  age
 1    Tom    4
 2  Jerry   15
```

`-l` 数字左对齐

```
$ echo "no name age\n1 Tom 4\n2 Jerry 15" | columnrs -l
no   name  age
1     Tom  4
2   Jerry  15
```