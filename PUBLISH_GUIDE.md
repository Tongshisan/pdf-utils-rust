# 📦 发布指南

## 🎯 更新的 publish.sh 功能

### 新增功能

1. **✅ npm 登录检查**

   - 自动检查是否已登录 npm
   - 显示当前登录用户
   - 未登录时提示并退出

2. **🔧 自动修复包名**

   - 检查生成的包名是否正确
   - 如果包名不是 `@tongshisan/pdf-utils-rust`，自动修复
   - 确保每次发布使用正确的包名

3. **🔍 版本冲突检查**

   - 发布前检查版本是否已存在
   - 避免重复发布同一版本
   - 提示更新版本号

4. **📊 详细信息显示**

   - 显示包名、版本、描述
   - 显示 WASM 文件大小
   - 更清晰的输出格式

5. **💡 发布后提示**
   - 提供 Git 提交和标签命令
   - 完整的后续步骤指导

## 🚀 使用方法

### 基本使用

```bash
./publish.sh
```

脚本会：

1. 检查 npm 登录状态
2. 清理并重新构建
3. 验证和修复包配置
4. 显示包信息
5. 检查版本冲突
6. 询问是否发布
7. 发布到 npm
8. 提供后续步骤建议

### 首次使用

如果还没登录 npm：

```bash
npm login
# 然后运行
./publish.sh
```

## 📝 完整发布流程

### 1. 修改代码后

```bash
# 1. 测试代码
cargo test

# 2. 更新版本号（在 Cargo.toml）
# version = "0.1.1" -> "0.1.2"

# 3. 运行发布脚本
./publish.sh
```

### 2. 脚本输出示例

```
🚀 准备发布 @tongshisan/pdf-utils-rust

🔑 检查 npm 登录状态...
✅ 已登录为: tongshisan

🧹 清理旧构建...
🔨 构建 WASM 包...

🔍 验证包配置...
✅ 包名正确

📦 包信息：
  "name": "@tongshisan/pdf-utils-rust",
  "version": "0.1.2",
  "description": "PDF and image processing utilities..."

📊 文件大小：
  WASM: 1.0M (pdf_utils_rust_bg.wasm)

📌 当前版本: 0.1.2
🔍 检查版本是否已存在...

是否发布到 NPM? (y/n)
```

### 3. 发布成功后

按照脚本提示的命令：

```bash
# 提交代码
git add .
git commit -m 'chore: release v0.1.2'

# 打标签
git tag v0.1.2

# 推送
git push origin main --tags
```

## ⚠️ 常见场景处理

### 场景 1: 忘记更新版本号

```
⚠️  版本 0.1.1 已经发布过了！

是否要更新版本号? (y/n) y

请在 Cargo.toml 中更新版本号，然后重新运行此脚本
```

**解决**：

1. 编辑 `Cargo.toml`
2. 更新 `version = "0.1.2"`
3. 重新运行 `./publish.sh`

### 场景 2: 包名自动修复

```
⚠️  包名不正确: pdf-utils-rust
🔧 正在修复包名...
✅ 包名已修复为: @tongshisan/pdf-utils-rust
```

这是正常的，脚本会自动修复。

### 场景 3: 未登录 npm

```
❌ 未登录 npm，请先运行: npm login
```

**解决**：

```bash
npm login
# 输入账号密码
./publish.sh
```

## 🔧 配置文件说明

### wasm-pack.toml

```toml
[package]
name = "@tongshisan/pdf-utils-rust"
```

这个文件告诉 wasm-pack 使用正确的包名。如果没有这个文件，wasm-pack 会使用 Cargo.toml 中的包名（没有 scope）。

### Cargo.toml

```toml
[package]
name = "pdf-utils-rust"
version = "0.1.1"  # 👈 每次发布前更新这里
```

## 📊 版本号规范

遵循 [语义化版本](https://semver.org/lang/zh-CN/)：

- **0.1.0 → 0.1.1**: Bug 修复（PATCH）
- **0.1.0 → 0.2.0**: 新功能（MINOR）
- **0.1.0 → 1.0.0**: 重大更改（MAJOR）

### 示例

| 更改类型   | 版本更新      | 说明     |
| ---------- | ------------- | -------- |
| 修复 bug   | 0.1.1 → 0.1.2 | 补丁版本 |
| 添加新功能 | 0.1.2 → 0.2.0 | 次版本   |
| 破坏性更改 | 0.2.0 → 1.0.0 | 主版本   |

## 🎯 最佳实践

### 发布前检查清单

- [ ] 代码已测试通过
- [ ] 更新了 CHANGELOG.md（如果有）
- [ ] 更新了版本号
- [ ] 更新了文档（如果需要）
- [ ] 本地测试了构建结果

### 发布后

- [ ] 验证包可以正常安装
- [ ] 创建 Git 标签
- [ ] 在 GitHub 创建 Release
- [ ] 更新文档网站（如果有）
- [ ] 通知用户更新

## 🆘 故障排除

### 问题：构建失败

```bash
# 检查 Rust 环境
rustc --version
cargo --version
wasm-pack --version

# 重新安装 wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### 问题：发布权限错误

```bash
# 检查登录用户
npm whoami

# 检查包权限
npm access ls-packages

# 重新登录
npm logout
npm login
```

### 问题：网络问题

```bash
# 使用代理
npm config set proxy http://proxy.company.com:8080
npm config set https-proxy http://proxy.company.com:8080

# 或使用淘宝镜像（仅查看，不影响发布）
npm config set registry https://registry.npmmirror.com
```

## 📚 相关命令

```bash
# 查看已发布的版本
npm view @tongshisan/pdf-utils-rust versions

# 查看包详情
npm view @tongshisan/pdf-utils-rust

# 标记版本为废弃
npm deprecate @tongshisan/pdf-utils-rust@0.1.0 "请升级到最新版本"

# 撤销发布（24小时内）
npm unpublish @tongshisan/pdf-utils-rust@0.1.1

# 测试安装
npm install @tongshisan/pdf-utils-rust@latest
```

---

**祝发布顺利！** 🚀
