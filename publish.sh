#!/bin/bash
set -e

echo "🚀 准备发布 @tongshisan/pdf-utils-rust"
echo ""

# 检查是否已登录 npm
echo "🔑 检查 npm 登录状态..."
if ! npm whoami &> /dev/null; then
    echo "❌ 未登录 npm，请先运行: npm login"
    exit 1
fi
echo "✅ 已登录为: $(npm whoami)"
echo ""

# 1. 清理
echo "🧹 清理旧构建..."
rm -rf pkg target

# 2. 构建
echo "🔨 构建 WASM 包..."
wasm-pack build --target bundler --out-dir pkg

# 3. 验证并修复包名
echo ""
echo "🔍 验证包配置..."
cd pkg

# 检查包名是否正确
PACKAGE_NAME=$(node -p "require('./package.json').name")
if [ "$PACKAGE_NAME" != "@tongshisan/pdf-utils-rust" ]; then
    echo "⚠️  包名不正确: $PACKAGE_NAME"
    echo "🔧 正在修复包名..."
    # 使用 node 脚本修复包名
    node -e "const fs=require('fs');const pkg=require('./package.json');pkg.name='@tongshisan/pdf-utils-rust';fs.writeFileSync('package.json',JSON.stringify(pkg,null,2));"
    echo "✅ 包名已修复为: @tongshisan/pdf-utils-rust"
fi

# 4. 显示包信息
echo ""
echo "📦 包信息："
cat package.json | grep -E '"name"|"version"|"description"'
echo ""
echo "📊 文件大小："
ls -lh *.wasm | awk '{print "  WASM: " $5 " (" $9 ")"}'
echo ""

# 5. 获取当前版本
CURRENT_VERSION=$(node -p "require('./package.json').version")
echo "📌 当前版本: $CURRENT_VERSION"

# 6. 检查版本是否已发布
echo "🔍 检查版本是否已存在..."
if npm view @tongshisan/pdf-utils-rust@$CURRENT_VERSION version &> /dev/null; then
    echo "⚠️  版本 $CURRENT_VERSION 已经发布过了！"
    echo ""
    read -p "是否要更新版本号? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo ""
        echo "请在 Cargo.toml 中更新版本号，然后重新运行此脚本"
        cd ..
        exit 1
    fi
fi

# 7. 询问是否发布
echo ""
read -p "是否发布到 NPM? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    echo ""
    echo "🚀 发布中..."
    npm publish --access public
    
    echo ""
    echo "✅ 发布成功！"
    echo ""
    echo "📝 验证："
    echo "  npm view @tongshisan/pdf-utils-rust"
    echo "  npm install @tongshisan/pdf-utils-rust@latest"
    echo ""
    echo "🌐 查看："
    echo "  https://www.npmjs.com/package/@tongshisan/pdf-utils-rust"
    echo ""
    echo "💡 建议下一步："
    echo "  1. 提交代码: git add . && git commit -m 'chore: release v$CURRENT_VERSION'"
    echo "  2. 打标签: git tag v$CURRENT_VERSION"
    echo "  3. 推送: git push origin main --tags"
else
    echo ""
    echo "❌ 已取消发布"
fi

cd ..

