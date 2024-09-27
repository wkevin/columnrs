# columnrs

A command-line tool for formatting tabular data into columns table. Just like `column` terminal command in linux, but can align right.

## Table of Contents

- [columnrs](#columnrs)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)

## Installation

You can install columnrs using Cargo, the Rust package manager. Run the following command:

```
$ cargo install columnrs
```

## Usage

Numeric columns are right-aligned by default:

```
$ echo "no name age\n1 Tom 4" | columnrs
no  name  age
 1  Tom     4
```

```
$ echo "no name age\n1 Tom 4\n2 Jerry 15" | columnrs
no  name   age
 1  Tom      4
 2  Jerry   15
```

`-l` change to left-aligned:

```
$ echo "no name age\n1 Tom 4\n2 Jerry 15" | columnrs -l
no  name   age
1   Tom    4
2   Jerry  15
```

```
$ cat dataset/data.csv|columnrs
NO  Extension         Publisher           Install  Trending  First_Published  Latest_Updated  Latest_Version
 1  claude-dev        saoudrizwan          63,921     60.86  2024-07-10       2024-09-26      1.9.7
 2  continue          Continue            329,208     17.68  2023-05-27       2024-09-25      0.9.212
 3  aide-pro          nicepkg              23,122     11.67  2024-07-02       2024-08-15      1.19.0
 ...
```
