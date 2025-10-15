# 使用示例

本目录包含使用 PDF & 图片处理工具的示例。

## 在线演示

### 启动本地服务器

由于浏览器的安全限制，需要通过 HTTP 服务器访问示例。

#### 使用 Python (推荐)

```bash
# 在项目根目录运行
python3 -m http.server 8000
```

#### 使用 Node.js (http-server)

```bash
npx http-server -p 8000
```

#### 使用 PHP

```bash
php -S localhost:8000
```

### 访问示例

在浏览器中打开：

```
http://localhost:8000/examples/index.html
```

## 示例说明

### index.html

完整的 Web 应用示例，包含所有功能：

1. **PDF 合并** - 上传多个 PDF 文件并合并
2. **PDF 分割** - 将 PDF 拆分为单独的页面
3. **图片转 PDF** - 将多张图片合并为 PDF
4. **图片格式转换** - 转换图片格式
5. **调整图片大小** - 改变图片尺寸
6. **图片压缩** - 减小图片文件大小
7. **旋转图片** - 旋转图片
8. **图片信息** - 查看图片详细信息

## 自定义使用

你可以参考 `index.html` 中的代码，在自己的项目中集成这些功能：

```javascript
import init, * as wasm from "../pkg/web/pdf_utils_rust.js";

// 初始化 WASM 模块
await init();

// 使用功能
const merged = wasm.merge_pdfs([pdf1, pdf2]);
```

## 注意事项

1. **构建 WASM 包**: 确保先运行 `./build.sh` 构建 WASM 包
2. **文件路径**: 注意 JavaScript 导入路径要正确指向 WASM 文件
3. **CORS**: 如果遇到 CORS 错误，确保通过 HTTP 服务器访问，而不是直接打开文件
4. **浏览器兼容性**: 需要支持 WebAssembly 的现代浏览器

## 性能提示

- 处理大文件时可能需要一些时间，请耐心等待
- 建议单次处理的文件不要过大（建议 < 50MB）
- 图片转 PDF 时，图片会被自动调整以适应 A4 页面大小
