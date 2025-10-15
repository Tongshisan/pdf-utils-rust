# 使用指南

## 🚀 快速开始

### 1. 安装依赖

首先确保安装了 Rust 和 wasm-pack：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### 2. 构建项目

```bash
# 克隆或进入项目目录
cd pdf-utils-rust

# 给构建脚本添加执行权限（如果还没有）
chmod +x build.sh

# 运行构建脚本
./build.sh
```

构建完成后，会在 `pkg` 目录下生成三个版本：

- `pkg/web/` - 浏览器版本
- `pkg/nodejs/` - Node.js 版本
- `pkg/bundler/` - 打包工具版本

### 3. 运行示例

```bash
# 启动本地服务器
python3 -m http.server 8000

# 或使用 npm 的方式
npm run serve
```

然后在浏览器中访问：`http://localhost:8000/examples/index.html`

## 💡 详细使用方法

### 在 HTML 中使用

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>PDF 工具示例</title>
  </head>
  <body>
    <input type="file" id="pdfInput" multiple accept=".pdf" />
    <button onclick="mergePdfs()">合并 PDF</button>

    <script type="module">
      import init, * as wasm from "./pkg/web/pdf_utils_rust.js";

      let wasmModule;

      async function initWasm() {
        wasmModule = await init();
        console.log("WASM 加载成功");
      }

      initWasm();

      window.mergePdfs = async function () {
        const files = document.getElementById("pdfInput").files;
        const pdfArrays = [];

        for (let file of files) {
          const buffer = await file.arrayBuffer();
          pdfArrays.push(new Uint8Array(buffer));
        }

        try {
          const merged = wasm.merge_pdfs(pdfArrays);

          // 下载合并后的 PDF
          const blob = new Blob([merged], { type: "application/pdf" });
          const url = URL.createObjectURL(blob);
          const a = document.createElement("a");
          a.href = url;
          a.download = "merged.pdf";
          a.click();

          alert("合并成功！");
        } catch (e) {
          alert("错误：" + e);
        }
      };
    </script>
  </body>
</html>
```

### 在 Node.js 中使用

```javascript
const fs = require("fs");
const wasm = require("./pkg/nodejs/pdf_utils_rust");

async function main() {
  // 读取 PDF 文件
  const pdf1 = fs.readFileSync("file1.pdf");
  const pdf2 = fs.readFileSync("file2.pdf");

  // 合并 PDF
  console.log("正在合并 PDF...");
  const merged = wasm.merge_pdfs([pdf1, pdf2]);
  fs.writeFileSync("merged.pdf", merged);
  console.log("✅ 合并完成！");

  // 分割 PDF
  console.log("正在分割 PDF...");
  const pages = wasm.split_pdf(pdf1);
  pages.forEach((page, index) => {
    fs.writeFileSync(`page_${index + 1}.pdf`, page);
  });
  console.log(`✅ 已分割为 ${pages.length} 个文件！`);

  // 图片转 PDF
  console.log("正在将图片转换为 PDF...");
  const img1 = fs.readFileSync("image1.jpg");
  const img2 = fs.readFileSync("image2.jpg");
  const pdf = wasm.images_to_pdf([img1, img2]);
  fs.writeFileSync("images.pdf", pdf);
  console.log("✅ 转换完成！");

  // 图片处理
  console.log("正在处理图片...");
  const image = fs.readFileSync("photo.jpg");

  // 调整大小
  const resized = wasm.resize_image(image, 800, 600, true);
  fs.writeFileSync("resized.jpg", resized);

  // 压缩图片
  const compressed = wasm.compress_image(image, 60);
  fs.writeFileSync("compressed.jpg", compressed);

  // 转换格式
  const converted = wasm.convert_image_format(image, "png", null);
  fs.writeFileSync("converted.png", converted);

  console.log("✅ 图片处理完成！");
}

main().catch(console.error);
```

### 在 React 中使用

```jsx
import React, { useEffect, useState } from "react";
import init, * as wasm from "./pkg/bundler/pdf_utils_rust";

function PdfMerger() {
  const [wasmLoaded, setWasmLoaded] = useState(false);

  useEffect(() => {
    init().then(() => {
      setWasmLoaded(true);
      console.log("WASM 加载成功");
    });
  }, []);

  const handleMerge = async (files) => {
    if (!wasmLoaded) {
      alert("WASM 模块还未加载完成");
      return;
    }

    const pdfArrays = [];
    for (let file of files) {
      const buffer = await file.arrayBuffer();
      pdfArrays.push(new Uint8Array(buffer));
    }

    try {
      const merged = wasm.merge_pdfs(pdfArrays);

      // 触发下载
      const blob = new Blob([merged], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = "merged.pdf";
      a.click();

      alert("合并成功！");
    } catch (e) {
      alert("错误：" + e);
    }
  };

  return (
    <div>
      <h2>PDF 合并工具</h2>
      <input
        type="file"
        multiple
        accept=".pdf"
        onChange={(e) => handleMerge(e.target.files)}
        disabled={!wasmLoaded}
      />
      {!wasmLoaded && <p>正在加载...</p>}
    </div>
  );
}

export default PdfMerger;
```

### 在 Vue 中使用

```vue
<template>
  <div>
    <h2>PDF 合并工具</h2>
    <input
      type="file"
      multiple
      accept=".pdf"
      @change="handleMerge"
      :disabled="!wasmLoaded"
    />
    <p v-if="!wasmLoaded">正在加载...</p>
  </div>
</template>

<script>
import init, * as wasm from "./pkg/bundler/pdf_utils_rust";

export default {
  name: "PdfMerger",
  data() {
    return {
      wasmLoaded: false,
      wasm: null,
    };
  },
  async mounted() {
    this.wasm = await init();
    this.wasmLoaded = true;
    console.log("WASM 加载成功");
  },
  methods: {
    async handleMerge(event) {
      const files = event.target.files;
      if (files.length < 2) {
        alert("请选择至少 2 个 PDF 文件");
        return;
      }

      const pdfArrays = [];
      for (let file of files) {
        const buffer = await file.arrayBuffer();
        pdfArrays.push(new Uint8Array(buffer));
      }

      try {
        const merged = wasm.merge_pdfs(pdfArrays);

        // 触发下载
        const blob = new Blob([merged], { type: "application/pdf" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "merged.pdf";
        a.click();

        alert("合并成功！");
      } catch (e) {
        alert("错误：" + e);
      }
    },
  },
};
</script>
```

## 🔧 常见问题

### 1. WASM 文件加载失败

**问题**: 浏览器报错 "Failed to fetch wasm file"

**解决方法**:

- 确保通过 HTTP 服务器访问，不要直接打开 HTML 文件
- 检查 WASM 文件路径是否正确
- 确保服务器正确设置了 WASM 的 MIME 类型

### 2. 内存不足

**问题**: 处理大文件时出现内存错误

**解决方法**:

- 分批处理文件
- 减小单个文件大小
- 增加浏览器内存限制（不推荐）

### 3. PDF 合并后样式丢失

**问题**: 合并后的 PDF 样式或格式有问题

**解决方法**:

- 确保输入的 PDF 文件格式正确
- 某些复杂的 PDF 特性可能不被支持
- 考虑使用其他 PDF 库或工具

### 4. 图片质量下降

**问题**: 图片转换或压缩后质量不理想

**解决方法**:

- 调整质量参数（提高 quality 值）
- 使用 PNG 格式保存（无损）
- 不要重复压缩同一张图片

## 📊 性能建议

1. **文件大小限制**

   - 单个 PDF 文件：建议 < 20MB
   - 单张图片：建议 < 10MB
   - 批量处理：总大小建议 < 50MB

2. **浏览器选择**

   - Chrome/Edge：性能最佳
   - Firefox：良好
   - Safari：一般

3. **优化技巧**
   - 预先压缩大文件
   - 使用合适的图片格式
   - 避免同时处理多个大文件

## 🎯 最佳实践

1. **错误处理**: 始终使用 try-catch 包裹 WASM 调用
2. **加载状态**: 显示加载指示器，提升用户体验
3. **进度反馈**: 对于大文件，提供处理进度
4. **内存管理**: 处理完后及时释放资源
5. **格式验证**: 处理前验证文件格式和大小

## 🔗 更多资源

- [完整 API 文档](README.md#api-文档)
- [示例代码](examples/)
- [常见问题](https://github.com/yourusername/pdf-utils-rust/issues)

## 💬 获取帮助

如果遇到问题：

1. 查看文档和示例代码
2. 搜索已有的 Issues
3. 创建新的 Issue 描述问题
4. 加入讨论社区

---

祝使用愉快！🎉
