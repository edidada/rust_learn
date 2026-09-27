# Git 分支重设计方案（Rust 笔记仓库）

> 供选择：每个方案给出结构、`checkout -b` 命令、优缺点与迁移步骤，文末有对比表与推荐组合。
> 选定后回复方案字母（可组合，如 `A+E`）即可执行。

## 一、现状盘点

- **本地分支**：`main`、`2015`、`2018`、`2021`、`2024`
- **main**：`56a2ca0 Merge branch '2024'`，`edition = "2024"` 单包，领先 `gitee/master` 54 个提交
- **四分支尖端**：`2015=4c9ca60`、`2018=237a4b2`、`2021=2a9a911`、`2024=f068f01`（均已推 origin）
- **缺口**：main 含 `f068f01`（2024 demo）与 `929b66c`（特性 md），但 **不含** `4c9ca60`、`237a4b2`、`2a9a911`（三个较早的分支 demo 提交，链未再跑）
- **远端**：origin / gitee / codeup 三份镜像；`main` 当前跟踪 `gitee/master`（`gitee/HEAD→gitee/master`、`origin/HEAD→origin/main` 不一致）
- **CI**：`.github/workflows/rust.yml` 触发分支为 main/master/2015/2018/2021/2024

### 既有痛点（本轮实践实录）

1. **链式 merge 手工、单向**（2015→2018→2021→2024）：修复无法下行，2018 的 `edition_2018.rs` 缺陷只能手动回同步
2. **`Cargo.toml` edition 行 + `[[bin]]` 注释门槛**：每次 merge 都要人肉判断取舍
3. **共享文件跨分支双改必冲突**：README 本轮实冲突（main 版 vs 重写版）；笔记 md 本质是全版本通用内容，却散落 4 份、各自漂移
4. **Cargo 硬约束**：一个 package 只能一个 `edition`——单分支演示多 Edition 必须 workspace 多包，否则只能靠分支
5. **远端 ×3、HEAD 指向混乱**：分支模型成本被放大三倍

---

## 方案 A：单分支 main + Workspace 多包（按 Edition 分 crate）

**核心**：砍掉四个分支；workspace 里每个 Edition 一个 package，各自持有自己的 `edition` 值。笔记 md 全部只存 main 一份。

```
main
├── Cargo.toml            # [workspace] members = crates/e2015..e2024, notes
├── Rust版本新特性.md / 分支文档…    # 笔记，唯一可信源
└── crates/
    ├── e2015/Cargo.toml  # edition = "2015"
    ├── e2018/Cargo.toml  # edition = "2018"
    ├── e2021/Cargo.toml  # edition = "2021"
    └── e2024/Cargo.toml  # edition = "2024"（承接现有练习/演示）
```

- 运行：`cargo run -p e2018 --bin edition_2018`；CI 一次跑 `cargo build --workspace` 覆盖全部 Edition
- **`[[bin]]` 注释门槛彻底消失**（每个包自己就是对应 edition）
- 分支工作流（配方案 E）：`git checkout -b note/<主题> main` → 写完 merge 回 main 删支

**迁移步骤（草案）**

```powershell
git checkout -b chore/workspace main        # 在分支上重构，main 保持可回退
# 建 crates/e2015..e2024，移动 src/bin 演示，各写 edition 的 Cargo.toml
cargo build --workspace                     # 全绿后
git checkout main && git merge chore/workspace
git tag archive/2015 4c9ca60                # 归档旧分支（可选）
git tag archive/2018 237a4b2
git tag archive/2021 2a9a911
git tag archive/2024 f068f01
git branch -d 2015 2018 2021 2024
```

| 优点 | 缺点 |
|---|---|
| 冲突归零、笔记单一源、CI 一把全验 | 一次性重构量最大 |
| 无链式 merge、无注释门槛 | 失去"checkout 即纯 Edition"体验（改用 `-p` 切换） |
| 长期维护成本最低 | 目录布局大改，历史路径检索受影响 |

---

## 方案 B：保留四分支 + 链式合并自动化（最小改动）

**核心**：拓扑不动，把本轮手工做的事固化成脚本与规则文档。

- 可选重命名使其自解释：
  ```powershell
  git checkout -b edition-2015 2015   # 旧名可删
  git checkout -b edition-2018 2018
  git checkout -b edition-2021 2021
  git checkout -b edition-2024 2024
  ```
- 新增 `scripts/merge-chain.ps1`：按序 2015→2018→2021→2024→main，冲突时按规则文件处理
- 规则文档化：`Cargo.toml` 永远取目标分支值；共享文件（README/演示文件）只允许在**最高分支**修改后上行；低分支不改共享文件
- 可选 CI：push 到低分支后自动跑下一环 merge

| 优点 | 缺点 |
|---|---|
| 零迁移；保留"checkout = 纯 Edition"体验 | 冲突根因未除：语义冲突（README 类）脚本救不了 |
| 脚本降低手工出错率 | 修复仍难下行；规则要人长期遵守 |
| | 远端仍 4 支 ×3 镜像，长期成本最高 |

---

## 方案 C：主干开发 + 旧 Edition 冻结为快照（trunk + frozen）

**核心**：只有 main 是"活"的；旧 Edition 分支完成使命后**冻结归档**，不再保证编译、不进 CI。

```powershell
git tag archive/edition-2015 4c9ca60
git tag archive/edition-2018 237a4b2
git tag archive/edition-2021 2a9a911
git tag archive/edition-2024 f068f01
git branch -D 2015 2018 2021 2024          # 远端同步删除
# 以后新主题笔记：
git checkout -b note/<主题> main
```

- main 可保持单包 `edition = "2024"`（只维护最新 Edition 的可编译性）
- 需要回看旧状态：`git checkout -b look-2015 archive/edition-2015`（临时，看完即删）
- 旧版特性讲解靠 `Rust版本新特性.md` + 快照代码（不承诺编译）

| 优点 | 缺点 |
|---|---|
| 分支数最少、冲突为零、维护最省 | 旧 Edition 代码不再持续验证（可能悄悄坏掉） |
| tag 永久锚定历史，比分支更轻 | "每个版本都能跑"的展示力丧失 |
| | 若想同仓演示多 Edition，仍需叠加方案 A 的 workspace |

---

## 方案 D：双轨——main 笔记主干 + 四个单向 demo 分支

**核心**：笔记与演示分离。main 只承载笔记（和通用练习）；每个 Edition 一个**独立单包** demo 分支，只从 main 单向接收笔记更新，互相之间**永不 merge**（消灭链）。

```powershell
git checkout -b demo-2015 main    # 每支 = 独立 crate，edition 对应，无 [[bin]] 门槛
git checkout -b demo-2018 main
git checkout -b demo-2021 main
git checkout -b demo-2024 main
```

- 规则：demo 分支**不改 md**（md 归 main），main 改笔记后 `git merge main` 到各 demo 即可，永远单向、无冲突
- 每个 demo 分支自身单 edition 单包 → 无注释门槛、独立 CI

| 优点 | 缺点 |
|---|---|
| 无链式冲突；各 demo 独立可编译 | 5 个常驻分支 ×3 远端（分支数未降） |
| 笔记单一源（main） | 笔记与演示分居两处，"边读边跑"要切分支 |
| 迁移量中等 | demo 分支仍需各自 CI |

---

## 方案 E：主题分支工作流（按笔记主题，不按 Edition）——可与 A/C 组合

**核心**：分支表达"正在写哪篇笔记"，而非"哪个 Edition"。Edition 归 workspace 包（需 A 的结构）或仅 2024（配 C）。

```powershell
git checkout -b note/closures main      # 写"闭包与捕获"笔记+示例
# 写完：
git checkout main && git merge note/closures && git branch -d note/closures
git checkout -b note/lifetime/main      # 下一篇
```

- 分支短命（小时/天级），永远从 main 切、写完即并回删
- 适配笔记仓库本质：分支 = 草稿纸

| 优点 | 缺点 |
|---|---|
| 最贴合"笔记仓库"心智；main 始终完整 | 单独用不了：Edition 编译差异必须先由 A/C 解决 |
| 分支即草稿，无长期拓扑负担 | 需要养成"短分支快合并"习惯 |

---

## 二、权衡对比表

| 维度 | A workspace 单分支 | B 链式+自动化 | C 主干+冻结 | D 双轨 | E 主题分支 |
|---|---|---|---|---|---|
| 迁移成本 | 高（重构目录） | **低** | 中（打 tag 删支） | 中 | 依赖 A/C |
| 合并冲突频率 | **无** | 常态（脚本只救一半） | **无** | 低（单向） | 低 |
| 笔记单一可信源 | ✅ | ❌ 4 份漂移 | ✅ | ✅ main | ✅ |
| 旧 Edition 持续可编译 | ✅ CI 全包 | ✅ | ❌ 冻结 | ✅ 各支自跑 | 随 A |
| "checkout=纯 Edition" | ❌ 改 `-p` | ✅ | ❌ | ✅ demo 支 | ❌ |
| 常驻分支数（每远端） | 1+短支 | 5 | 1+tag | 5 | 1+短支 |
| `[[bin]]` 注释门槛 | **消失** | 仍在 | 仅 main（2024） | **消失** | 随 A |
| 长期维护负担 | 低 | **高** | **低** | 中 | 低 |
| 适配笔记仓库程度 | 高 | 中 | 高 | 中 | **最高** |

---

## 三、推荐组合与附带整理

**推荐：A + E + C 的归档思路**（结构用 A，工作流用 E，现有四分支按 C 打 tag 归档后删除）。
理由：本仓库是**笔记**而非多版本产品线——冲突与链式合并全部源自"共享内容 × 四分支"；workspace 把 Edition 差异收缩进 `crates/`，主题分支把版本控制还给"写笔记"本身。

偏好"每个版本随时 checkout 可跑"则选 **D**（或 A+保留四支的折中）；只想立刻止损不想重构则选 **B**。

**无论选哪个，先做三件事：**

1. 把 `4c9ca60`、`237a4b2`、`2a9a911` 三个 demo 提交并入 main（链跑或 cherry-pick），避免重构时丢工作
2. 统一远端语义：确定 `main` 跟踪对象（现状跟踪 `gitee/master`），三 remote 分清"主从/镜像"
3. 按选定方案收缩 `rust.yml` 的触发分支

**选择方式**：回复方案字母（单选或组合，如 `A+E`、`D`、`B+1`）。
