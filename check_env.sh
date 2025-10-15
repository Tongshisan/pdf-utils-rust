#!/bin/bash

echo "🔍 检查开发环境..."
echo ""

# 检查 Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust: $(rustc --version)"
else
    echo "❌ Rust 未安装"
    echo "   安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

# 检查 Cargo
if command -v cargo &> /dev/null; then
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Cargo 未安装"
fi

# 检查 wasm-pack
if command -v wasm-pack &> /dev/null; then
    echo "✅ wasm-pack: $(wasm-pack --version)"
else
    echo "❌ wasm-pack 未安装"
    echo "   方式 1（推荐）: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    echo "   方式 2（如果有 npm）: npm install -g wasm-pack"
    echo "   方式 3（macOS）: brew install wasm-pack"
fi

# 检查 WASM 目标
if command -v rustup &> /dev/null; then
    if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
        echo "✅ wasm32-unknown-unknown 目标已安装"
    else
        echo "❌ wasm32-unknown-unknown 目标未安装"
        echo "   安装命令: rustup target add wasm32-unknown-unknown"
    fi
fi

# 检查 Python（用于运行示例服务器）
if command -v python3 &> /dev/null; then
    echo "✅ Python3: $(python3 --version)"
else
    echo "⚠️  Python3 未安装（可选，用于运行示例）"
fi

# 检查 Node.js（可选，用于 npm 安装 wasm-pack）
if command -v node &> /dev/null; then
    echo "✅ Node.js: $(node --version)"
else
    echo "ℹ️  Node.js 未安装（可选，可用于安装 wasm-pack）"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📝 总结："
if command -v rustc &> /dev/null && command -v wasm-pack &> /dev/null; then
    echo "✅ 环境配置完成！可以开始构建项目。"
    echo ""
    echo "🚀 下一步："
    echo "   1. 运行: ./build.sh"
    echo "   2. 或: wasm-pack build --target web --out-dir pkg/web"
    echo "   3. 启动服务器: python3 -m http.server 8000"
    echo "   4. 打开浏览器: http://localhost:8000/examples/index.html"
else
    echo "❌ 环境未完全配置，请按照上面的提示安装缺失的工具。"
    echo ""
    echo "🔧 推荐的安装顺序："
    echo "   1. 安装 Rust:"
    echo "      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "      source \$HOME/.cargo/env"
    echo ""
    echo "   2. 安装 wasm-pack（选择一种方式）:"
    echo "      方式 A: npm install -g wasm-pack"
    echo "      方式 B: brew install wasm-pack"
    echo "      方式 C: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    echo ""
    echo "   3. 添加 WASM 目标:"
    echo "      rustup target add wasm32-unknown-unknown"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

