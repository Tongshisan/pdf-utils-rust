# 图片水印去除功能使用指南

本文档详细介绍如何使用 `pdf-utils-rust` 中的水印去除功能。

## 📋 目录

- [功能概述](#功能概述)
- [使用方法](#使用方法)
  - [方法一：区域修复](#方法一区域修复)
  - [方法二：颜色过滤](#方法二颜色过滤)
  - [方法三：淡化水印](#方法三淡化水印)
- [最佳实践](#最佳实践)
- [常见问题](#常见问题)
- [示例代码](#示例代码)

## 功能概述

水印去除功能提供了三种不同的方法来处理图片中的水印：

### 1. 区域修复 (`remove_watermark_by_region`)

**适用场景**：水印位置固定且已知

**工作原理**：

- 指定水印所在的矩形区域
- 使用智能算法修复该区域
- 提供三种修复方法：
  - `average`: 平均值填充
  - `blur`: 模糊处理
  - `median`: 中值滤波

**优点**：

- 效果稳定可靠
- 不影响图片其他区域
- 适合各种类型的水印

**缺点**：

- 需要手动指定水印位置和大小
- 对复杂背景的效果可能不够完美

### 2. 颜色过滤 (`remove_watermark_by_color`)

**适用场景**：水印为特定颜色（如灰色、半透明白色等）

**工作原理**：

- 识别指定颜色范围内的像素
- 将这些像素替换为白色或周围像素的平均值
- 通过容差参数控制颜色匹配的严格程度

**优点**：

- 可以一次性去除图片中所有相同颜色的水印
- 适合文字水印
- 不需要精确指定位置

**缺点**：

- 只适用于纯色或单色水印
- 可能误删除与水印颜色相近的正常内容
- 对渐变色水印效果较差

### 3. 淡化水印 (`fade_watermark`)

**适用场景**：深色水印，希望减轻但不完全去除

**工作原理**：

- 增加指定区域的像素亮度
- 使深色水印变淡
- 不会影响图片的主要内容

**优点**：

- 简单快速
- 对图片质量影响最小
- 适合轻度水印

**缺点**：

- 无法完全去除水印
- 可能使该区域过亮
- 不适合浅色水印

## 使用方法

### 方法一：区域修复

#### JavaScript 示例

```javascript
import init, * as wasm from 'pdf-utils-rust';

// 初始化 WASM
await init();

// 读取图片
const imageBytes = new Uint8Array(await file.arrayBuffer());

// 去除水印 - 使用平均值填充
const result1 = wasm.remove_watermark_by_region(
	imageBytes,
	100, // x 坐标：水印左上角的 x 位置
	100, // y 坐标：水印左上角的 y 位置
	200, // 宽度：水印的宽度
	100, // 高度：水印的高度
	'average' // 方法：average, blur, median
);

// 去除水印 - 使用模糊处理
const result2 = wasm.remove_watermark_by_region(
	imageBytes,
	100,
	100,
	200,
	100,
	'blur'
);

// 去除水印 - 使用中值滤波
const result3 = wasm.remove_watermark_by_region(
	imageBytes,
	100,
	100,
	200,
	100,
	'median'
);

// 保存结果
const blob = new Blob([result1], { type: 'image/png' });
// ... 下载或显示
```

#### 参数说明

| 参数          | 类型         | 说明                                        |
| ------------- | ------------ | ------------------------------------------- |
| `image_bytes` | `Uint8Array` | 原始图片数据                                |
| `x`           | `number`     | 水印区域左上角 x 坐标                       |
| `y`           | `number`     | 水印区域左上角 y 坐标                       |
| `width`       | `number`     | 水印区域宽度                                |
| `height`      | `number`     | 水印区域高度                                |
| `method`      | `string`     | 修复方法：`"average"`, `"blur"`, `"median"` |

#### 修复方法对比

**Average（平均值填充）**

- 使用水印区域边界像素的平均颜色填充
- 适合：纯色背景、渐变背景
- 处理速度：快 ⚡⚡⚡
- 效果质量：中等 ★★★☆☆

**Blur（模糊处理）**

- 对水印区域应用强模糊效果
- 适合：复杂背景、纹理背景
- 处理速度：中等 ⚡⚡☆
- 效果质量：较好 ★★★★☆

**Median（中值滤波）**

- 使用周围像素的中值替换
- 适合：有噪点的水印、细小水印
- 处理速度：慢 ⚡☆☆
- 效果质量：好 ★★★★★

### 方法二：颜色过滤

#### JavaScript 示例

```javascript
// 去除灰色水印（RGB: 200, 200, 200）
const result = wasm.remove_watermark_by_color(
	imageBytes,
	200, // R (红色) 0-255
	200, // G (绿色) 0-255
	200, // B (蓝色) 0-255
	30, // 容差：允许的颜色偏差范围
	true // 是否用白色替换（false 则用周围像素平均值）
);

// 去除白色半透明水印
const result2 = wasm.remove_watermark_by_color(
	imageBytes,
	255,
	255,
	255, // 白色
	20, // 小容差
	false // 使用周围像素填充
);

// 去除深灰色文字水印
const result3 = wasm.remove_watermark_by_color(
	imageBytes,
	128,
	128,
	128, // 深灰色
	40, // 较大容差
	true // 白色填充
);
```

#### 参数说明

| 参数                 | 类型         | 说明                                              |
| -------------------- | ------------ | ------------------------------------------------- |
| `image_bytes`        | `Uint8Array` | 原始图片数据                                      |
| `target_r`           | `number`     | 目标颜色的红色分量 (0-255)                        |
| `target_g`           | `number`     | 目标颜色的绿色分量 (0-255)                        |
| `target_b`           | `number`     | 目标颜色的蓝色分量 (0-255)                        |
| `tolerance`          | `number`     | 颜色容差 (0-100，建议 20-40)                      |
| `replace_with_white` | `boolean`    | `true`: 用白色填充; `false`: 用周围像素平均值填充 |

#### 如何选择目标颜色

1. **使用取色工具**：

   - 在图片编辑软件中使用吸管工具获取水印颜色
   - 浏览器开发者工具的取色器
   - 在线取色工具

2. **常见水印颜色**：

   - 灰色水印：`RGB(200, 200, 200)` 或 `RGB(128, 128, 128)`
   - 白色半透明：`RGB(255, 255, 255)` 或 `RGB(240, 240, 240)`
   - 黑色半透明：`RGB(50, 50, 50)` 或 `RGB(80, 80, 80)`

3. **容差设置建议**：
   - 精确匹配：10-20（只去除非常接近的颜色）
   - 一般使用：30-40（适合大多数情况）
   - 宽松匹配：50-70（可能会误删除其他内容）

### 方法三：淡化水印

#### JavaScript 示例

```javascript
// 淡化深色水印
const result = wasm.fade_watermark(
	imageBytes,
	100, // x 坐标
	100, // y 坐标
	200, // 宽度
	100, // 高度
	50 // 亮度增加值（0-150）
);

// 轻度淡化
const result2 = wasm.fade_watermark(
	imageBytes,
	50,
	50,
	300,
	150,
	30 // 较小的亮度增加
);

// 强力淡化
const result3 = wasm.fade_watermark(
	imageBytes,
	50,
	50,
	300,
	150,
	100 // 较大的亮度增加
);
```

#### 参数说明

| 参数                  | 类型         | 说明                     |
| --------------------- | ------------ | ------------------------ |
| `image_bytes`         | `Uint8Array` | 原始图片数据             |
| `x`                   | `number`     | 水印区域左上角 x 坐标    |
| `y`                   | `number`     | 水印区域左上角 y 坐标    |
| `width`               | `number`     | 水印区域宽度             |
| `height`              | `number`     | 水印区域高度             |
| `brightness_increase` | `number`     | 亮度增加值 (建议 20-100) |

#### 亮度参数建议

- **20-40**：轻微淡化，适合浅色背景
- **50-80**：中度淡化，适合大多数情况
- **90-120**：强力淡化，适合深色水印
- **120+**：极度淡化，可能导致区域过亮

## 最佳实践

### 1. 选择合适的方法

```
┌─────────────────────────────────────────────┐
│          水印类型决策树                      │
├─────────────────────────────────────────────┤
│                                             │
│  水印位置固定且已知？                        │
│  ├─ 是 → 使用【区域修复】                    │
│  └─ 否 ↓                                    │
│                                             │
│  水印是纯色或单色？                          │
│  ├─ 是 → 使用【颜色过滤】                    │
│  └─ 否 ↓                                    │
│                                             │
│  只需要减轻水印？                            │
│  ├─ 是 → 使用【淡化水印】                    │
│  └─ 否 → 尝试【区域修复 + 模糊】             │
│                                             │
└─────────────────────────────────────────────┘
```

### 2. 组合使用多种方法

```javascript
// 先淡化，再用颜色过滤精细处理
let result = wasm.fade_watermark(imageBytes, 100, 100, 200, 100, 50);
result = wasm.remove_watermark_by_color(result, 230, 230, 230, 30, true);
```

### 3. 批量处理

```javascript
// 处理多个区域
const regions = [
	{ x: 100, y: 100, width: 200, height: 50 },
	{ x: 300, y: 500, width: 150, height: 40 },
];

let result = imageBytes;
for (const region of regions) {
	result = wasm.remove_watermark_by_region(
		result,
		region.x,
		region.y,
		region.width,
		region.height,
		'blur'
	);
}
```

### 4. 优化性能

```javascript
// 对于大图片，先缩小处理再放大
const resized = wasm.resize_image(imageBytes, 800, 600, true);
const processed = wasm.remove_watermark_by_region(
	resized,
	50,
	50,
	100,
	50,
	'blur'
);
const final = wasm.resize_image(processed, 1600, 1200, true);
```

## 常见问题

### Q1: 处理后图片质量下降怎么办？

**A**:

- 使用 PNG 格式保存以避免压缩损失
- 对于区域修复，尝试使用 `"blur"` 方法而不是 `"average"`
- 减小处理区域的范围
- 避免重复处理同一区域

### Q2: 水印去除不干净？

**A**:

- **区域修复**：扩大水印区域范围，确保完全覆盖水印
- **颜色过滤**：增加容差值，或多次使用不同的目标颜色
- **淡化水印**：增加亮度值
- 尝试组合使用多种方法

### Q3: 误删除了正常内容？

**A**:

- **颜色过滤**：减小容差值，使颜色匹配更精确
- **区域修复**：精确指定水印位置，不要包含正常内容
- 使用 `replace_with_white=false` 以使用周围像素填充

### Q4: 如何找到水印的精确位置？

**A**:

```javascript
// 在图片上绘制参考框
const canvas = document.createElement('canvas');
const ctx = canvas.getContext('2d');
const img = new Image();
img.onload = () => {
	canvas.width = img.width;
	canvas.height = img.height;
	ctx.drawImage(img, 0, 0);

	// 绘制红框标记水印位置
	ctx.strokeStyle = 'red';
	ctx.lineWidth = 2;
	ctx.strokeRect(100, 100, 200, 100);

	// 显示坐标
	ctx.fillStyle = 'red';
	ctx.font = '16px Arial';
	ctx.fillText('(100, 100)', 100, 95);
};
img.src = URL.createObjectURL(imageBlob);
```

### Q5: 处理速度太慢？

**A**:

- 避免使用 `"median"` 方法处理大区域
- 对大图片先进行缩小处理
- 使用 Web Worker 在后台线程处理
- 考虑分批处理多张图片

### Q6: 某些格式的图片处理失败？

**A**:

- 确保图片格式被支持（JPEG, PNG, BMP, GIF）
- 某些 WebP 格式可能不被完全支持
- 先转换为 PNG 格式再处理：
  ```javascript
  const png = wasm.convert_image_format(imageBytes, 'png', 100);
  const result = wasm.remove_watermark_by_region(png, x, y, w, h, 'blur');
  ```

## 示例代码

### 完整的 Web 应用示例

```html
<!DOCTYPE html>
<html lang="zh-CN">
	<head>
		<meta charset="UTF-8" />
		<title>水印去除工具</title>
	</head>
	<body>
		<h1>水印去除工具</h1>

		<input type="file" id="fileInput" accept="image/*" />

		<div>
			<h3>区域设置</h3>
			<label>X: <input type="number" id="x" value="100" /></label>
			<label>Y: <input type="number" id="y" value="100" /></label>
			<label>宽: <input type="number" id="width" value="200" /></label>
			<label>高: <input type="number" id="height" value="100" /></label>

			<select id="method">
				<option value="average">平均值</option>
				<option value="blur">模糊</option>
				<option value="median">中值滤波</option>
			</select>

			<button id="processBtn">处理</button>
		</div>

		<div>
			<h3>原图</h3>
			<img id="original" style="max-width: 400px;" />
		</div>

		<div>
			<h3>处理后</h3>
			<img id="processed" style="max-width: 400px;" />
			<button id="downloadBtn">下载</button>
		</div>

		<script type="module">
			import init, * as wasm from './pkg/web/pdf_utils_rust.js';

			let imageBytes = null;
			let processedBytes = null;

			// 初始化
			await init();

			// 文件选择
			document
				.getElementById('fileInput')
				.addEventListener('change', async (e) => {
					const file = e.target.files[0];
					if (!file) return;

					imageBytes = new Uint8Array(await file.arrayBuffer());

					// 显示原图
					const blob = new Blob([imageBytes], { type: file.type });
					document.getElementById('original').src = URL.createObjectURL(blob);
				});

			// 处理按钮
			document.getElementById('processBtn').addEventListener('click', () => {
				if (!imageBytes) {
					alert('请先选择图片');
					return;
				}

				const x = parseInt(document.getElementById('x').value);
				const y = parseInt(document.getElementById('y').value);
				const width = parseInt(document.getElementById('width').value);
				const height = parseInt(document.getElementById('height').value);
				const method = document.getElementById('method').value;

				try {
					processedBytes = wasm.remove_watermark_by_region(
						imageBytes,
						x,
						y,
						width,
						height,
						method
					);

					// 显示处理后的图片
					const blob = new Blob([processedBytes], { type: 'image/png' });
					document.getElementById('processed').src = URL.createObjectURL(blob);

					alert('处理完成！');
				} catch (error) {
					alert('处理失败：' + error);
				}
			});

			// 下载按钮
			document.getElementById('downloadBtn').addEventListener('click', () => {
				if (!processedBytes) {
					alert('没有可下载的图片');
					return;
				}

				const blob = new Blob([processedBytes], { type: 'image/png' });
				const url = URL.createObjectURL(blob);
				const a = document.createElement('a');
				a.href = url;
				a.download = 'watermark-removed.png';
				a.click();
				URL.revokeObjectURL(url);
			});
		</script>
	</body>
</html>
```

### Node.js 批量处理示例

```javascript
const fs = require('fs');
const path = require('path');
const wasm = require('pdf-utils-rust');

// 批量处理目录中的所有图片
function batchRemoveWatermark(inputDir, outputDir, config) {
	const files = fs.readdirSync(inputDir);

	for (const file of files) {
		if (!/\.(jpg|jpeg|png|bmp|gif)$/i.test(file)) continue;

		console.log(`处理: ${file}`);

		const inputPath = path.join(inputDir, file);
		const outputPath = path.join(outputDir, file);

		try {
			const imageBytes = fs.readFileSync(inputPath);

			const result = wasm.remove_watermark_by_region(
				imageBytes,
				config.x,
				config.y,
				config.width,
				config.height,
				config.method
			);

			fs.writeFileSync(outputPath, result);
			console.log(`✓ 完成: ${file}`);
		} catch (error) {
			console.error(`✗ 失败: ${file} - ${error.message}`);
		}
	}
}

// 使用示例
batchRemoveWatermark('./input', './output', {
	x: 100,
	y: 100,
	width: 200,
	height: 100,
	method: 'blur',
});
```

## 技术支持

如果您在使用过程中遇到问题，请：

1. 查看本文档的[常见问题](#常见问题)部分
2. 查看项目的 [GitHub Issues](https://github.com/yourusername/pdf-utils-rust/issues)
3. 提交新的 Issue 并附上：
   - 使用的方法和参数
   - 错误信息
   - 示例图片（如果可能）

## 贡献

欢迎提交 PR 来改进水印去除算法！可能的改进方向：

- 添加更多修复算法（如 Telea 算法、NS 算法）
- 支持自动检测水印位置
- 添加深度学习模型支持
- 优化处理速度

## 许可证

MIT License

---

💡 **提示**: 访问 `examples/watermark-remover.html` 查看完整的交互式演示！

