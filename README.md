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
