# Project Structure
The `tileset-celluar-automata` contains Rust code for heavy calculations in the cellular automata simulation. To add new cell types, update the `CellRules` enum and its implementations.

Game logic and UI are handled in GDScript. To avoid global state issues, the codebase uses an `EventBus` script to pass signals between loosely connected nodes in the scenetree.

`scenes` folder contains scenes.
`scripts` folder contains scripts written in GDScript
`assets` folder contains assets.

# Project Setup

this project is built in Godot 4.4.1, make sure you have this version of godot installed.

you also need to have the rust programming language installed, this is critical since you need to make sure to run `cargo build --release` in the tileset-celluar-automata folder before first importing, and whenever you make changes to the `tileset-celluar-automata` code, the reason for building testing builds with the `--release` flag is because the unoptimized profile can cause an absurd amount of lag.

once you have built the rust library, import the project in godot 4.4.1

# CI

the project comes with a CI workflow which automatically compiles the rust code to run on browser and then builds the project, expect headaches if you edit it
without knowing what you are doing.
the CI workflow will upload the built webpage to the branch `gh-pages`, this is so that you can configure the github pages to deploy from the `gh-pages` branch and it will automatically update the page.

### Note to anyone who will be working on this.

The lead designer has decided that we are not trying to hit the deadline of the game jam, I'm too tired to keep working on this. I've tried my best here to document the codebase, and I wish you the best of luck.

# 项目结构 (translated with copilot)
`tileset-celluar-automata` 包含用于细胞自动机模拟中大量计算的 Rust 代码。要添加新的细胞类型，请更新 `CellRules` Enum 其实现。

游戏逻辑和 UI 由 GDScript 处理。为避免全局状态问题，代码库使用 `EventBus` 脚本在场景树中松散连接的节点之间传递信号。

`scenes` 文件夹包含场景。
`scripts` 文件夹包含用 GDScript 编写的脚本。
`assets` 文件夹包含资源。

# 项目设置

本项目基于 Godot 4.4.1 构建，请确保已安装此版本的 Godot。

你还需要安装 Rust 编程语言，这非常关键。你需要在首次导入前以及每次更改 `tileset-celluar-automata` 代码后，在该文件夹下运行 `cargo build --release`。使用 `--release` 标志构建测试版本是因为未优化的配置会导致极大的卡顿。

构建好 Rust 库后，在 Godot 4.4.1 中导入项目。

# CI

本项目自带 CI 工作流，会自动将 Rust 代码编译为可在浏览器运行的版本，然后构建项目。如果你不了解 CI，请不要随意修改，否则可能会遇到麻烦。
CI 工作流会将构建好的网页上传到 `gh-pages` 分支，这样你可以将 GitHub Pages 配置为从 `gh-pages` 分支部署，页面会自动更新。

### 给所有将要参与本项目的人的说明

主设计师已决定我们不再追赶游戏创作马拉松的截止日期，我已经太累了，无法继续工作。我已经尽力记录了代码库，祝你们好运。
