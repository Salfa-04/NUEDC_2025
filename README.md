# 快速开始模板

复制目录快速开始。

## 使用方法

### 复制示例目录

要在自己的项目中使用这些示例，您需要复制整个目录：

```bash
cp /path/to/directory -r /path/to/project
```

然后重命名目录内的示例：

```bash
mv /path/to/project/example -r /path/to/project/temporary
```

### 修改配置字段

在使用示例之前，您需要修改以下文件中的关键字段：

1. 在 `Cargo.toml` 中：
    - 将 `<CHIP>` 修改为目标芯片 : 如 `stm32g473cb`

2. 在 `.cargo/config.toml` 中：
    - 将 `<CHIP>` 修改为目标芯片 : 如 `STM32G473CB`
    - 将 `<TARGET_TRIPLE>` 修改目标平台的三元组 : 如 `thumbv7em-none-eabihf`

3. 在 `openocd.cfg` 中：
    - 将 `<CHIP>` 修改为目标芯片所代表的配置文件 : 如 `stm32g4x`

4. 在 `example/Cargo.toml` 中：
    - 将 `<NAME>` 修改为该文件所在目录的命名 : 如 `temporary`

5. 在 `example/.vscode/launch.json` 中：
    - 将 `<CHIP>` 修改为目标芯片 : 如 `STM32G473CB`
    - 将 `<TARGET_TRIPLE>` 修改目标平台的三元组 : 如 `thumbv7em-none-eabihf`

6. 修改 `memory.x`，或删除并为 `embassy-stm32` 添加 `memory-x` Feature

## 下载程序

### 使用 `Probe-rs`工具

在 `.cargo/config.toml` 中，修改 `runner` 中的 `attach` 为 `run`

使用 `Probe-rs` 下载：

```bash
cargo run # in debug
    # or
cargo rr # in release
```

使用 `OpenOCD` 下载：

```bash
cargo b && openocd # in debug
    # or
cargo br && openocd # in release

# Attach
cargo rr # or `cargo r` in debug
```
