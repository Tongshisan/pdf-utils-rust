# PDF & 图片处理工具 (Rust + WebAssembly)

一个强大的文件处理工具库，使用 Rust 编写并编译为 WebAssembly，可在浏览器中直接运行。

## ✨ 功能特性

### PDF 处理

- 📑 **PDF 合并** - 将多个 PDF 文件合并为一个
- ✂️ **PDF 分割** - 将 PDF 分割为单独的页面
- 📄 **按范围分割** - 按指定页码范围提取 PDF 页面
- 📊 **获取页数** - 快速获取 PDF 文档的总页数

### 图片处理

- 🖼️ **图片转 PDF** - 将多张图片合并为一个 PDF 文件
- 🔄 **格式转换** - 支持 JPEG、PNG、BMP、GIF 等格式转换
- 📐 **调整大小** - 改变图片尺寸，支持保持宽高比
- 🗜️ **图片压缩** - 压缩图片以减小文件大小
- 🔄 **图片旋转** - 旋转图片 90°、180° 或 270°
- ✂️ **图片裁剪** - 裁剪图片的指定区域
- ℹ️ **图片信息** - 获取图片的详细信息（尺寸、格式、颜色类型等）
- 🧹 **水印去除** - 智能去除图片水印（支持区域修复、颜色过滤、淡化等方法）

## 🚀 快速开始

### 前置要求

- [Rust](https://www.rust-lang.org/tools/install) (最新稳定版)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 wasm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## 📦 构建

### 使用构建脚本（推荐）

```bash
chmod +x build.sh
./build.sh
```

这将生成三个版本的 WASM 包：

- `pkg/web/` - 用于浏览器
- `pkg/nodejs/` - 用于 Node.js
- `pkg/bundler/` - 用于 Webpack/Rollup 等打包工具

### 手动构建

```bash
wasm-pack build --scope tongshisan
```

#### 构建 Web 版本

```bash
wasm-pack build --target web --out-dir pkg/web
```

#### 构建 Node.js 版本

```bash
wasm-pack build --target nodejs --out-dir pkg/nodejs
```

#### 构建 Bundler 版本

```bash
wasm-pack build --target bundler --out-dir pkg/bundler
```

## 💻 使用示例

### 在浏览器中使用

1. 构建 WASM 包：

```bash
./build.sh
```

2. 启动本地服务器（例如使用 Python）：

```bash
python3 -m http.server 8000
```

3. 在浏览器中打开：

```
http://localhost:8000/examples/index.html
```

### 在 JavaScript 中使用

```javascript
import init, * as wasm from './pkg/web/pdf_utils_rust.js';

async function main() {
	// 初始化 WASM 模块
	await init();

	// PDF 合并
	const pdf1 = new Uint8Array(await file1.arrayBuffer());
	const pdf2 = new Uint8Array(await file2.arrayBuffer());
	const merged = wasm.merge_pdfs([pdf1, pdf2]);

	// PDF 分割
	const pages = wasm.split_pdf(pdfBytes);

	// 图片转 PDF
	const img1 = new Uint8Array(await image1.arrayBuffer());
	const img2 = new Uint8Array(await image2.arrayBuffer());
	const pdf = wasm.images_to_pdf([img1, img2]);

	// 图片格式转换
	const converted = wasm.convert_image_format(imageBytes, 'png', 85);

	// 调整图片大小
	const resized = wasm.resize_image(imageBytes, 800, 600, true);

	// 压缩图片
	const compressed = wasm.compress_image(imageBytes, 75);

	// 旋转图片
	const rotated = wasm.rotate_image(imageBytes, 90);

	// 裁剪图片
	const cropped = wasm.crop_image(imageBytes, 100, 100, 400, 300);

	// 获取图片信息
	const info = JSON.parse(wasm.get_image_info(imageBytes));
	console.log(info); // { width, height, format, color_type }

	// 去除图片水印 - 区域修复方法
	const removedByRegion = wasm.remove_watermark_by_region(
		imageBytes,
		100, // x 坐标
		100, // y 坐标
		200, // 宽度
		100, // 高度
		'average' // 方法: average, blur, median
	);

	// 去除图片水印 - 颜色过滤方法
	const removedByColor = wasm.remove_watermark_by_color(
		imageBytes,
		200, // 目标红色值
		200, // 目标绿色值
		200, // 目标蓝色值
		30, // 容差
		true // 是否用白色替换
	);

	// 淡化水印
	const faded = wasm.fade_watermark(
		imageBytes,
		100, // x 坐标
		100, // y 坐标
		200, // 宽度
		100, // 高度
		50 // 亮度增加值
	);

	// 获取 PDF 页数
	const pageCount = wasm.get_pdf_page_count(pdfBytes);
}

main();
```

### 在 Node.js 中使用

```javascript
const fs = require('fs');
const wasm = require('pdf-utils-rust');

// 读取 PDF 文件
const pdf1 = fs.readFileSync('file1.pdf');
const pdf2 = fs.readFileSync('file2.pdf');

// 合并 PDF
const merged = wasm.merge_pdfs([pdf1, pdf2]);
fs.writeFileSync('merged.pdf', merged);

// 分割 PDF
const pages = wasm.split_pdf(pdf1);
pages.forEach((page, index) => {
	fs.writeFileSync(`page_${index + 1}.pdf`, page);
});

// 图片转 PDF
const img1 = fs.readFileSync('image1.jpg');
const img2 = fs.readFileSync('image2.jpg');
const pdf = wasm.images_to_pdf([img1, img2]);
fs.writeFileSync('images.pdf', pdf);
```

### 在 Next.js 中使用

#### 安装

```bash
npm install pdf-utils-rust
# 或
yarn add pdf-utils-rust
# 或
pnpm add pdf-utils-rust
```

#### Next.js 13+ (App Router)

创建一个客户端组件：

```tsx
// app/components/PdfTools.tsx
'use client';

import { useState, useEffect } from 'react';
import * as wasm from 'pdf-utils-rust';

export default function PdfTools() {
	const [wasmLoaded, setWasmLoaded] = useState(false);
	const [processing, setProcessing] = useState(false);

	useEffect(() => {
		// 在客户端初始化 WASM
		wasm
			.default()
			.then(() => {
				setWasmLoaded(true);
				console.log('✅ WASM 模块加载成功');
			})
			.catch((err) => {
				console.error('❌ WASM 加载失败:', err);
			});
	}, []);

	const handleMergePdfs = async (files: FileList) => {
		if (!wasmLoaded || files.length < 2) return;

		setProcessing(true);
		try {
			const pdfArrays = [];
			for (let i = 0; i < files.length; i++) {
				const buffer = await files[i].arrayBuffer();
				pdfArrays.push(new Uint8Array(buffer));
			}

			const merged = wasm.merge_pdfs(pdfArrays);

			// 下载合并后的 PDF
			const blob = new Blob([merged], { type: 'application/pdf' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = 'merged.pdf';
			a.click();
			URL.revokeObjectURL(url);

			alert('✅ PDF 合并成功！');
		} catch (error) {
			console.error('错误:', error);
			alert('❌ 处理失败: ' + error);
		} finally {
			setProcessing(false);
		}
	};

	const handleImageToPdf = async (files: FileList) => {
		if (!wasmLoaded || files.length < 1) return;

		setProcessing(true);
		try {
			const imageArrays = [];
			for (let i = 0; i < files.length; i++) {
				const buffer = await files[i].arrayBuffer();
				imageArrays.push(new Uint8Array(buffer));
			}

			const pdf = wasm.images_to_pdf(imageArrays);

			const blob = new Blob([pdf], { type: 'application/pdf' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = 'images.pdf';
			a.click();
			URL.revokeObjectURL(url);

			alert('✅ 图片转 PDF 成功！');
		} catch (error) {
			console.error('错误:', error);
			alert('❌ 处理失败: ' + error);
		} finally {
			setProcessing(false);
		}
	};

	if (!wasmLoaded) {
		return <div>正在加载 WASM 模块...</div>;
	}

	return (
		<div className="space-y-6">
			<div className="card">
				<h2 className="text-xl font-bold mb-4">PDF 合并</h2>
				<input
					type="file"
					multiple
					accept=".pdf"
					onChange={(e) => e.target.files && handleMergePdfs(e.target.files)}
					disabled={processing}
					className="file-input"
				/>
			</div>

			<div className="card">
				<h2 className="text-xl font-bold mb-4">图片转 PDF</h2>
				<input
					type="file"
					multiple
					accept="image/*"
					onChange={(e) => e.target.files && handleImageToPdf(e.target.files)}
					disabled={processing}
					className="file-input"
				/>
			</div>

			{processing && <div>处理中...</div>}
		</div>
	);
}
```

在页面中使用：

```tsx
// app/page.tsx
import PdfTools from './components/PdfTools';

export default function Home() {
	return (
		<main className="container mx-auto p-8">
			<h1 className="text-3xl font-bold mb-8">PDF 工具</h1>
			<PdfTools />
		</main>
	);
}
```

#### Next.js 配置（如果需要）

如果遇到 WASM 加载问题，可以在 `next.config.js` 中添加配置：

```javascript
// next.config.js
/** @type {import('next').NextConfig} */
const nextConfig = {
	webpack: (config, { isServer }) => {
		// 添加 WASM 支持
		config.experiments = {
			...config.experiments,
			asyncWebAssembly: true,
		};

		// 确保 WASM 文件被正确处理
		config.module.rules.push({
			test: /\.wasm$/,
			type: 'webassembly/async',
		});

		return config;
	},
};

module.exports = nextConfig;
```

#### Next.js 12 及以下（Pages Router）

```tsx
// pages/pdf-tools.tsx
import { useState, useEffect } from 'react';
import type { NextPage } from 'next';
import dynamic from 'next/dynamic';

// 禁用 SSR 以避免 WASM 在服务器端加载
const PdfToolsComponent = dynamic(() => import('../components/PdfTools'), {
	ssr: false,
});

const PdfToolsPage: NextPage = () => {
	return (
		<div className="container">
			<h1>PDF 工具</h1>
			<PdfToolsComponent />
		</div>
	);
};

export default PdfToolsPage;
```

#### TypeScript 支持

包已经包含了 TypeScript 类型定义，可以直接使用：

```typescript
import * as wasm from 'pdf-utils-rust';

// 类型会自动推断
const mergePdfs: (pdfs: Uint8Array[]) => Uint8Array = wasm.merge_pdfs;
const splitPdf: (pdf: Uint8Array) => Uint8Array[] = wasm.split_pdf;
```

#### 完整示例：PDF 工具应用

```tsx
// components/PdfProcessor.tsx
'use client';

import { useState, useEffect } from 'react';
import * as wasm from 'pdf-utils-rust';

type ProcessingState = 'idle' | 'loading' | 'processing' | 'success' | 'error';

export default function PdfProcessor() {
	const [wasmLoaded, setWasmLoaded] = useState(false);
	const [state, setState] = useState<ProcessingState>('loading');
	const [message, setMessage] = useState('');

	useEffect(() => {
		wasm
			.default()
			.then(() => {
				setWasmLoaded(true);
				setState('idle');
			})
			.catch((err) => {
				console.error('WASM 加载失败:', err);
				setState('error');
				setMessage('WASM 模块加载失败');
			});
	}, []);

	const downloadFile = (data: Uint8Array, filename: string, type: string) => {
		const blob = new Blob([data], { type });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = filename;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
	};

	const processPdfs = async (
		files: FileList,
		operation: 'merge' | 'split' | 'img2pdf'
	) => {
		if (!wasmLoaded) return;

		setState('processing');
		setMessage('处理中...');

		try {
			const fileArrays = [];
			for (let i = 0; i < files.length; i++) {
				const buffer = await files[i].arrayBuffer();
				fileArrays.push(new Uint8Array(buffer));
			}

			switch (operation) {
				case 'merge':
					const merged = wasm.merge_pdfs(fileArrays);
					downloadFile(merged, 'merged.pdf', 'application/pdf');
					setMessage(`✅ 成功合并 ${files.length} 个 PDF 文件`);
					break;

				case 'split':
					const pages = wasm.split_pdf(fileArrays[0]);
					pages.forEach((page: Uint8Array, index: number) => {
						downloadFile(page, `page_${index + 1}.pdf`, 'application/pdf');
					});
					setMessage(`✅ PDF 已分割为 ${pages.length} 个文件`);
					break;

				case 'img2pdf':
					const pdf = wasm.images_to_pdf(fileArrays);
					downloadFile(pdf, 'images.pdf', 'application/pdf');
					setMessage(`✅ ${files.length} 张图片已转换为 PDF`);
					break;
			}

			setState('success');
			setTimeout(() => setState('idle'), 3000);
		} catch (error) {
			console.error('处理失败:', error);
			setState('error');
			setMessage(`❌ 处理失败: ${error}`);
		}
	};

	if (state === 'loading') {
		return (
			<div className="flex items-center justify-center p-8">
				<div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
				<span className="ml-4">加载中...</span>
			</div>
		);
	}

	return (
		<div className="max-w-4xl mx-auto p-6 space-y-6">
			<div className="bg-white rounded-lg shadow p-6">
				<h2 className="text-2xl font-bold mb-4">PDF 合并</h2>
				<p className="text-gray-600 mb-4">选择多个 PDF 文件进行合并</p>
				<input
					type="file"
					multiple
					accept=".pdf"
					onChange={(e) =>
						e.target.files && processPdfs(e.target.files, 'merge')
					}
					disabled={state === 'processing'}
					className="w-full p-2 border rounded"
				/>
			</div>

			<div className="bg-white rounded-lg shadow p-6">
				<h2 className="text-2xl font-bold mb-4">PDF 分割</h2>
				<p className="text-gray-600 mb-4">将 PDF 分割为单独的页面</p>
				<input
					type="file"
					accept=".pdf"
					onChange={(e) =>
						e.target.files && processPdfs(e.target.files, 'split')
					}
					disabled={state === 'processing'}
					className="w-full p-2 border rounded"
				/>
			</div>

			<div className="bg-white rounded-lg shadow p-6">
				<h2 className="text-2xl font-bold mb-4">图片转 PDF</h2>
				<p className="text-gray-600 mb-4">将多张图片合并为一个 PDF</p>
				<input
					type="file"
					multiple
					accept="image/*"
					onChange={(e) =>
						e.target.files && processPdfs(e.target.files, 'img2pdf')
					}
					disabled={state === 'processing'}
					className="w-full p-2 border rounded"
				/>
			</div>

			{message && (
				<div
					className={`p-4 rounded ${
						state === 'success'
							? 'bg-green-100 text-green-800'
							: state === 'error'
							? 'bg-red-100 text-red-800'
							: 'bg-blue-100 text-blue-800'
					}`}
				>
					{message}
				</div>
			)}
		</div>
	);
}
```

#### 注意事项

1. **客户端渲染**: WASM 只能在客户端运行，必须使用 `'use client'` 指令或 `dynamic` 导入禁用 SSR

2. **异步初始化**: WASM 模块需要异步加载，确保在使用前调用 `wasm.default()`

3. **文件大小**: 处理大文件时可能需要较长时间，建议添加加载指示器

4. **内存管理**: 处理完大文件后，及时清理不需要的数据

5. **错误处理**: 始终使用 try-catch 包裹 WASM 调用

6. **浏览器兼容性**: 确保目标浏览器支持 WebAssembly

## 📚 API 文档

### PDF 功能

#### `merge_pdfs(pdf_files: Vec<Uint8Array>) -> Vec<u8>`

合并多个 PDF 文件。

- **参数**: PDF 文件的字节数组列表
- **返回**: 合并后的 PDF 字节数组

#### `split_pdf(pdf_bytes: &[u8]) -> Array`

将 PDF 分割为单独的页面。

- **参数**: PDF 文件的字节数组
- **返回**: 包含每一页 PDF 的数组

#### `split_pdf_by_range(pdf_bytes: &[u8], page_ranges: &str) -> Vec<u8>`

按页码范围分割 PDF。

- **参数**:
  - `pdf_bytes`: PDF 文件的字节数组
  - `page_ranges`: 页码范围字符串，如 "1-3,5,7-9"
- **返回**: 提取的 PDF 字节数组

#### `get_pdf_page_count(pdf_bytes: &[u8]) -> usize`

获取 PDF 的总页数。

- **参数**: PDF 文件的字节数组
- **返回**: 页数

### 图片功能

#### `images_to_pdf(images: Vec<Uint8Array>) -> Vec<u8>`

将多张图片转换为 PDF。

- **参数**: 图片文件的字节数组列表
- **返回**: PDF 字节数组

#### `convert_image_format(image_bytes: &[u8], target_format: &str, quality: Option<u8>) -> Vec<u8>`

转换图片格式。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `target_format`: 目标格式 ("jpeg", "png", "bmp", "gif")
  - `quality`: JPEG 质量 (0-100，可选，默认 85)
- **返回**: 转换后的图片字节数组

#### `resize_image(image_bytes: &[u8], width: u32, height: u32, maintain_aspect_ratio: bool) -> Vec<u8>`

调整图片大小。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `width`: 目标宽度
  - `height`: 目标高度
  - `maintain_aspect_ratio`: 是否保持宽高比
- **返回**: 调整后的图片字节数组

#### `compress_image(image_bytes: &[u8], quality: u8) -> Vec<u8>`

压缩图片。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `quality`: 压缩质量 (0-100)
- **返回**: 压缩后的图片字节数组

#### `rotate_image(image_bytes: &[u8], degrees: i32) -> Vec<u8>`

旋转图片。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `degrees`: 旋转角度 (90, 180, 270)
- **返回**: 旋转后的图片字节数组

#### `crop_image(image_bytes: &[u8], x: u32, y: u32, width: u32, height: u32) -> Vec<u8>`

裁剪图片。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `x`: 起始 x 坐标
  - `y`: 起始 y 坐标
  - `width`: 裁剪宽度
  - `height`: 裁剪高度
- **返回**: 裁剪后的图片字节数组

#### `get_image_info(image_bytes: &[u8]) -> String`

获取图片信息。

- **参数**: 图片字节数组
- **返回**: JSON 字符串，包含 width, height, format, color_type

#### `remove_watermark_by_region(image_bytes: &[u8], x: u32, y: u32, width: u32, height: u32, method: &str) -> Vec<u8>`

基于区域修复去除水印。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `x`: 水印区域 x 坐标
  - `y`: 水印区域 y 坐标
  - `width`: 水印区域宽度
  - `height`: 水印区域高度
  - `method`: 修复方法
    - `"average"`: 使用周围像素平均值填充（适合纯色背景）
    - `"blur"`: 使用模糊处理（适合复杂背景）
    - `"median"`: 使用中值滤波（适合去除噪点）
- **返回**: 处理后的图片字节数组

#### `remove_watermark_by_color(image_bytes: &[u8], target_r: u8, target_g: u8, target_b: u8, tolerance: u8, replace_with_white: bool) -> Vec<u8>`

基于颜色过滤去除水印。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `target_r`: 目标红色值 (0-255)
  - `target_g`: 目标绿色值 (0-255)
  - `target_b`: 目标蓝色值 (0-255)
  - `tolerance`: 颜色容差 (0-100)
  - `replace_with_white`: 是否用白色替换（false 则使用周围像素平均值）
- **返回**: 处理后的图片字节数组

#### `fade_watermark(image_bytes: &[u8], x: u32, y: u32, width: u32, height: u32, brightness_increase: i32) -> Vec<u8>`

淡化水印（通过增加亮度）。

- **参数**:
  - `image_bytes`: 图片字节数组
  - `x`: 水印区域 x 坐标
  - `y`: 水印区域 y 坐标
  - `width`: 水印区域宽度
  - `height`: 水印区域高度
  - `brightness_increase`: 亮度增加值 (建议 0-150)
- **返回**: 处理后的图片字节数组

## 🛠️ 技术栈

- **Rust** - 核心处理逻辑
- **WebAssembly** - 编译目标
- **wasm-bindgen** - Rust 和 JavaScript 交互
- **lopdf** - PDF 处理库
- **image** - 图片处理库

## 📈 性能优势

- ⚡ **高性能** - Rust 编译为 WebAssembly，接近原生性能
- 🔒 **内存安全** - Rust 的所有权系统保证内存安全
- 🌐 **跨平台** - 可在任何支持 WebAssembly 的环境运行
- 📦 **轻量级** - 编译后的 WASM 文件体积小
- 🚀 **零依赖** - 无需后端服务器，完全在客户端处理

## ⚠️ 注意事项

### 关于 Word 转 PDF

目前纯 Rust + WASM 环境下，Word 转 PDF 功能实现较为复杂。推荐的替代方案：

1. **后端处理**: 使用 LibreOffice、Pandoc 等工具在服务器端转换
2. **在线 API**: 使用第三方 API 服务
3. **浏览器打印**: 使用浏览器的打印功能转换为 PDF

### 浏览器兼容性

需要支持 WebAssembly 的现代浏览器：

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## 🧪 测试

```bash
# 运行测试
cargo test

# 运行 WASM 测试
wasm-pack test --headless --firefox
```

## 📝 开发

```bash
# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 构建发布版本
cargo build --release
```

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

## 📄 许可证

MIT License

## 🔗 相关链接

- [Rust 官网](https://www.rust-lang.org/)
- [WebAssembly 官网](https://webassembly.org/)
- [wasm-pack 文档](https://rustwasm.github.io/docs/wasm-pack/)
- [lopdf 文档](https://docs.rs/lopdf/)
- [image 文档](https://docs.rs/image/)

## 📮 联系方式

如有问题或建议，欢迎提交 Issue。

---

⭐ 如果这个项目对你有帮助，欢迎 Star！
