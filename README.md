# rsb-cli
Read the Russian Synodal Bible (Синодальный Перевод) from the command line

## Installation
```
➜ git clone git@github.com:trevarj/rsb-cli.git
➜ cargo install --path rsb-cli/
```

## Usage

Use `-h` or `--help` to see a list of all arguments.
```
➜ rsb -h     
Usage: rsb [OPTIONS]

Read the Russian Synodal Bible from the command-line

Optional arguments:
  -h, --help  prints help message

Available commands:
  gen      Бытие
  ex       Исход
  lev      Левит
  ...
  rev      Откровение святого Иоанна Богослова
```

### Printing a single verse of a chapter of a book:
```
➜ rsb gen 1:1
[Gen 1:1]   В начале сотворил Бог небо и землю.
```

### Printing select verses of a chapter of a book:
```
➜ rsb gen 1:1-10
[Gen 1:1]   В начале сотворил Бог небо и землю.
[Gen 1:2]   Земля же была безвидна и пуста, и тьма над бездною, и Дух Божий
            носился над водою.
[Gen 1:3]   И сказал Бог: да будет свет. И стал свет.
...
[Gen 1:10]  И назвал Бог сушу землею, а собрание вод назвал морями. И увидел
            Бог, что _это_ хорошо.
```

### Printing an entire chapter of a book:
```
➜ rsb gen 1
```

### Printing an entire book:
```
➜ rsb gen
```