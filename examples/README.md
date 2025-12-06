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
http://localhost:8000/examples/watermark-remover.html  (水印去除专用工具)
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

### watermark-remover.html

专门的水印去除工具，提供三种去除方法：

1. **区域修复** - 指定水印位置，使用智能算法修复

   - 平均值填充（适合纯色背景）
   - 模糊处理（适合复杂背景）
   - 中值滤波（适合去除噪点）

2. **颜色过滤** - 去除特定颜色的水印

   - 支持 RGB 颜色选择
   - 可调节容差范围
   - 可选择填充方式（白色或周围像素）

3. **淡化水印** - 通过增加亮度减轻水印
   - 适合深色水印
   - 可调节亮度增加值
   - 对图片质量影响最小

**使用方法**：

- 上传带水印的图片
- 选择合适的去除方法
- 调整参数
- 实时预览效果
- 下载处理后的图片

详细使用说明请查看 [WATERMARK_REMOVAL_CN.md](../WATERMARK_REMOVAL_CN.md)

## 自定义使用

你可以参考 `index.html` 中的代码，在自己的项目中集成这些功能：

```javascript
import init, * as wasm from '../pkg/web/pdf_utils_rust.js';

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
