# 📦 NPM 包更新指南

## 当前信息

- 包名: `@tongshisan/pdf-utils-rust`
- 当前版本: `0.1.0`
- 建议新版本: `0.1.1` (bug 修复)

## 🚀 完整更新流程

### 步骤 1: 更新 Cargo.toml 版本

```bash
# 编辑 Cargo.toml，将版本从 0.1.0 改为 0.1.1
```

或运行：

```bash
# 手动更新版本号
sed -i '' 's/version = "0.1.0"/version = "0.1.1"/' Cargo.toml
```

### 步骤 2: 清理旧构建

```bash
rm -rf pkg target
```

### 步骤 3: 重新构建 WASM 包

```bash
# 构建 bundler 版本（用于 npm 发布）
wasm-pack build --target bundler --out-dir pkg

# 或者如果你需要其他版本
wasm-pack build --target web --out-dir pkg/web
wasm-pack build --target nodejs --out-dir pkg/nodejs
```

**注意**: 发布到 npm 通常使用 `bundler` 目标。

### 步骤 4: 检查生成的 package.json

```bash
cat pkg/package.json
```

确认：

- ✅ 版本号正确更新为 0.1.1
- ✅ 包名为 @tongshisan/pdf-utils-rust
- ✅ 包含所有必要文件

### 步骤 5: 测试构建结果

```bash
# 进入 pkg 目录
cd pkg

# 检查文件
ls -lh

# 应该看到：
# - pdf_utils_rust_bg.wasm (WASM 文件)
# - pdf_utils_rust.js (JavaScript 绑定)
# - pdf_utils_rust.d.ts (TypeScript 类型)
# - package.json (包配置)
```

### 步骤 6: 登录 NPM（如果还没登录）

```bash
npm login
# 或
npm login --registry=https://registry.npmjs.org/
```

输入你的 npm 账号信息：

- Username
- Password
- Email
- 2FA Code (如果启用了)

### 步骤 7: 发布到 NPM

```bash
# 在 pkg 目录下
cd pkg

# 发布（首次发布或公开包）
npm publish --access public

# 如果已经发布过，直接发布
npm publish
```

### 步骤 8: 验证发布

```bash
# 查看包信息
npm view @tongshisan/pdf-utils-rust

# 安装测试
npm install @tongshisan/pdf-utils-rust@latest
```

或访问：https://www.npmjs.com/package/@tongshisan/pdf-utils-rust

## 🔄 快速命令（一键更新）

创建一个更新脚本：

```bash
cat > update-and-publish.sh << 'EOF'
#!/bin/bash
set -e

echo "🔧 准备发布新版本..."

# 1. 清理
echo "🧹 清理旧构建..."
rm -rf pkg target

# 2. 构建
echo "🔨 构建 WASM 包..."
wasm-pack build --target bundler --out-dir pkg

# 3. 进入 pkg 目录
cd pkg

# 4. 显示包信息
echo ""
echo "📦 包信息："
cat package.json | grep -E '"name"|"version"'

# 5. 询问是否继续
echo ""
read -p "是否发布到 NPM? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    echo "🚀 发布中..."
    npm publish --access public
    echo "✅ 发布成功！"
    echo ""
    echo "📝 验证："
    echo "npm view @tongshisan/pdf-utils-rust"
else
    echo "❌ 已取消发布"
fi

cd ..
EOF

chmod +x update-and-publish.sh
```

然后运行：

```bash
./update-and-publish.sh
```

## 📝 版本号规范

根据 [语义化版本](https://semver.org/lang/zh-CN/)：

- **补丁版本 (0.1.0 → 0.1.1)**: Bug 修复
- **次版本 (0.1.0 → 0.2.0)**: 新功能（向后兼容）
- **主版本 (0.1.0 → 1.0.0)**: 重大更改（可能不兼容）

**本次更新建议**: `0.1.0 → 0.1.1` (修复了 PDF 处理的重大 bug)

## ⚠️ 常见问题

### 问题 1: 权限错误

```bash
npm ERR! code E403
npm ERR! 403 Forbidden
```

**解决**：

```bash
# 确保已登录
npm whoami

# 重新登录
npm logout
npm login

# 检查包名是否可用
npm view @tongshisan/pdf-utils-rust
```

### 问题 2: 版本已存在

```bash
npm ERR! code EPUBLISHCONFLICT
npm ERR! 403 You cannot publish over the previously published version
```

**解决**：更新版本号

```bash
# 在 Cargo.toml 中更新版本号
# 然后重新构建
```

### 问题 3: 需要 2FA 验证

如果启用了双因素认证：

```bash
npm publish --otp=123456  # 替换为你的 2FA 代码
```

### 问题 4: 构建失败

```bash
error: failed to parse manifest
```

**解决**：

```bash
# 检查 Cargo.toml 语法
cargo check

# 确保 Rust 和 wasm-pack 已安装
rustc --version
wasm-pack --version
```

## 🎯 发布检查清单

- [ ] 代码已测试，所有 bug 已修复
- [ ] 更新了 Cargo.toml 中的版本号
- [ ] 清理了旧的构建文件
- [ ] 成功构建 WASM 包
- [ ] 检查了 pkg/package.json 的内容
- [ ] 已登录 npm
- [ ] 本地测试了构建结果
- [ ] 准备好发布说明（可选）

## 📄 发布后

### 1. 更新 Git 标签

```bash
git add .
git commit -m "chore: bump version to 0.1.1 - fix PDF processing bugs"
git tag v0.1.1
git push origin main --tags
```

### 2. 创建 Release Notes (GitHub)

在 GitHub 上创建 Release，说明：

```markdown
## v0.1.1 - Bug Fixes

### 🐛 修复

- 修复了 `merge_pdfs` 函数无法正确合并 PDF 的问题
- 修复了 `split_pdf` 函数生成损坏文件的问题
- 修复了 `split_pdf_by_range` 函数文件大小异常的问题

### 🔧 技术改进

- 添加了 `deep_copy_object` 函数，正确处理 PDF 对象引用
- 完善了 PDF 文档结构构建逻辑

### 📦 如何使用

\`\`\`bash
npm install @tongshisan/pdf-utils-rust@latest
\`\`\`
```

### 3. 通知用户

如果你有用户群或文档站点，更新使用说明。

## 🔗 有用的命令

```bash
# 查看当前登录用户
npm whoami

# 查看包的所有版本
npm view @tongshisan/pdf-utils-rust versions

# 查看包的最新信息
npm info @tongshisan/pdf-utils-rust

# 撤销发布（24小时内）
npm unpublish @tongshisan/pdf-utils-rust@0.1.1

# 标记版本为废弃
npm deprecate @tongshisan/pdf-utils-rust@0.1.0 "包含严重 bug，请升级到 0.1.1"
```

---

**祝发布顺利！** 🎉
