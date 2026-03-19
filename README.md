# FAQ

## Консольные команды

### Проверка установленной версии Rust
>
> rustc --version

### Локальная документация
>
> rustup doc

### Запуск программы на Windows
>
> rustc main.rs
> .\main.exe

### Запуск программы на Linux/MacOS
>
> rustc main.rs
> ./main.exe

### Создание проекта (бинарного крейта) с помощью Cargo
>
> cargo new hello_world

### Создание библиотеки (библиотечного крейта) с помощью Cargo
>
> cargo new hello_world --lib

### Создание проекта Cargo без использования системы контроля версий
>
> cargo new --vcs=none hello_world

### Сборка Cargo проекта
>
> cargo build

### Сборка и запуск Cargo проекта
>
> cargo run

### Проверка компилируемости кода без создания исполняемого файла
>
> cargo check

### Сборка финальной версии (Release)
>
> cargo build --release

### Обновление пакета для получения новой версии
>
> cargo update

### Сборка локальной документации, предоставляемой всеми зависимостями в проекте
>
> cargo doc --open
