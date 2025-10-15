# 贡献指南

感谢你对本项目的关注！我们欢迎任何形式的贡献。

## 🤝 如何贡献

### 报告 Bug

如果你发现了 bug，请：

1. 检查 [Issues](https://github.com/yourusername/pdf-utils-rust/issues) 中是否已有相同问题
2. 如果没有，创建新的 Issue，包含：
   - 清晰的标题和描述
   - 重现步骤
   - 预期行为和实际行为
   - 环境信息（浏览器版本、操作系统等）
   - 如果可能，提供示例代码或文件

### 提出新功能

如果你有新功能的想法：

1. 创建 Feature Request Issue
2. 详细描述功能的用途和使用场景
3. 如果可能，提供设计方案或伪代码

### 提交代码

1. **Fork 项目**

   ```bash
   git clone https://github.com/yourusername/pdf-utils-rust.git
   cd pdf-utils-rust
   ```

2. **创建新分支**

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **编写代码**

   - 遵循 Rust 代码规范
   - 添加必要的注释
   - 编写测试（如果适用）

4. **测试代码**

   ```bash
   # 检查代码格式
   cargo fmt --check

   # 运行 linter
   cargo clippy

   # 运行测试
   cargo test

   # 构建 WASM
   ./build.sh
   ```

5. **提交更改**

   ```bash
   git add .
   git commit -m "feat: 添加新功能描述"
   ```

6. **推送到 GitHub**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **创建 Pull Request**
   - 提供清晰的 PR 描述
   - 引用相关的 Issue
   - 等待代码审查

## 📝 代码规范

### Rust 代码

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust API 设计指南
- 为公共 API 编写文档注释

### 提交信息

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

- `feat:` 新功能
- `fix:` Bug 修复
- `docs:` 文档更新
- `style:` 代码格式（不影响功能）
- `refactor:` 重构
- `test:` 添加测试
- `chore:` 构建过程或辅助工具的变动

示例：

```
feat: 添加 PDF 加密功能
fix: 修复图片旋转后的内存泄漏
docs: 更新 API 文档
```

## 🧪 测试

- 为新功能添加单元测试
- 确保所有测试通过
- 测试在不同浏览器中的兼容性

## 📚 文档

- 更新相关文档（README、API 文档等）
- 为新功能添加使用示例
- 保持中英文文档同步

## 🔍 代码审查

所有代码都需要经过审查：

- 至少一位维护者批准
- 通过所有自动化测试
- 代码质量符合规范
- 文档完整

## 💡 开发技巧

### 调试 WASM

1. 在浏览器控制台查看日志
2. 使用 `console_log!` 宏打印调试信息
3. 使用浏览器的 WASM 调试工具

### 性能优化

1. 使用 `cargo build --release` 构建优化版本
2. 使用 `wasm-opt` 进一步优化 WASM 文件
3. 避免不必要的内存分配

### 本地测试

```bash
# 构建并运行示例
./build.sh
python3 -m http.server 8000

# 在浏览器中打开
# http://localhost:8000/examples/index.html
```

## 🎯 优先级

当前优先级较高的任务：

- [ ] 添加更多图片处理功能（滤镜、调色等）
- [ ] 优化大文件处理性能
- [ ] 添加 PDF 加密/解密功能
- [ ] 改进错误处理和提示
- [ ] 添加更多单元测试

## 📞 联系方式

- 提交 Issue
- 创建 Discussion
- 发送 PR

## 📜 许可证

贡献的代码将采用与项目相同的 MIT 许可证。

---

再次感谢你的贡献！🎉
