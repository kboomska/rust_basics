# FAQ

## 🖥️ Консольные команды

### Проверка установленной версии Rust

```bash
rustc --version
```

### Обновление Rust

```bash
rustup update stable
```

### Локальная документация

```bash
rustup doc
```

### Запуск программы на Windows

```bash
rustc main.rs
.\main.exe
```

### Запуск программы на Linux/MacOS

```bash
rustc main.rs
./main.exe
```

### Создание проекта (бинарного крейта) с помощью Cargo

```bash
cargo new hello_world
```

### Создание библиотеки (библиотечного крейта) с помощью Cargo

```bash
cargo new hello_world --lib
```

### Создание проекта Cargo без использования системы контроля версий

```bash
cargo new --vcs=none hello_world
```

### Сборка Cargo проекта

```bash
cargo build
```

### Сборка и запуск Cargo проекта

```bash
cargo run
```

### Проверка компилируемости кода без создания исполняемого файла

```bash
cargo check
```

### Сборка финальной версии (Release)

```bash
cargo build --release
```

### Обновление пакета для получения новой версии

```bash
cargo update
```

### Сборка локальной документации, предоставляемой всеми зависимостями в проекте

```bash
cargo doc --open
```

### Запуск тестов

```bash
cargo test
```

### Последовательный запуск тестов

> ⚠️ **Использование разделителя "--" указывает, что следующие аргументы предназначены для двоичного файла программы, а не для cargo**

```bash
cargo test -- --test-threads=1
```

### Запуск тестов с последующим выводом результатов println! для успешно выполненных тестов

> ⚠️ **Для тестов, которые завершились с ошибкой, результаты println! выводятся всегда**

```bash
cargo test -- --show-output
```

### Выборочный запуск тестов

Для запуска одного конкретного теста необходимо указать его название при вызове `cargo test`.

> ⚠️ **Если указать часть имени теста, то будут запущены тесты, имена которых содержат указанное значение**

```bash
cargo test test_name
```

### Запуск только игнорируемых тестов

```bash
cargo test -- --ignored
```

### Запуск всех тестов, включая игнорируемые

```bash
cargo test -- --include-ignored
```

### Запуск всех тестов в конкретном файле интеграционного теста

```bash
cargo test --test integration_test
```

## 💡 Советы

### Уменьшение размера исполняемого файла за счет отключения раскрутки стека при панике

Необходимо добавить в [profile] раздел файла Cargo.toml:

```toml
[profile.release]
panic = 'abort'
```

### Отображение обратной трассировки panic! на Windows

```bash
$env:RUST_BACKTRACE=1
cargo run
```

### Отображение обратной трассировки panic! на Linux/MacOS

```bash
RUST_BACKTRACE=1 cargo run
```

### Устанавливаем переменную окружения и запускаем программу на Windows

> ⚠️ **Переменная окружения IGNORE_CASE сохранится до конца сеанса работы консоли**

```bash
$Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

### Отключаем переменную окружения на Windows

```bash
Remove-Item Env:IGNORE_CASE
```

### Устанавливаем переменную окружения и запускаем программу на Linux/MacOS

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```
