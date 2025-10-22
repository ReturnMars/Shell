#!/usr/bin/env node

import { execSync } from 'child_process';
import { existsSync, mkdirSync, rmSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 获取命令行参数
const args = process.argv.slice(2);
const command = args[0] || 'check';

// 目录管理函数
function clearCacheDir() {
  if (existsSync(cacheDir)) {
    rmSync(cacheDir, { recursive: true, force: true });
  }
  mkdirSync(cacheDir, { recursive: true });
}

function ensureCacheDir() {
  if (!existsSync(cacheDir)) {
    mkdirSync(cacheDir, { recursive: true });
  }
}

// 根据命令类型处理目录
const cacheDir = join(__dirname, '..', '.cache', 'oxlint');
if (command !== 'open') {
  clearCacheDir();
} else {
  ensureCacheDir();
}

// 处理函数
function handleCheck() {
  console.log('🔍 运行代码检查...');
  execSync('oxlint', { stdio: 'inherit' });
}

function handleFix() {
  console.log('🔧 自动修复代码问题...');
  execSync('oxlint --fix', { stdio: 'inherit' });
}

function handleReport() {
  console.log('📊 生成 JSON 报告...');
  execSync(`oxlint --format json > "${join(cacheDir, 'report.json')}"`, { stdio: 'inherit' });
  console.log('✅ JSON 报告已生成:', join(cacheDir, 'report.json'));
}

function handleHtml() {
  console.log('📊 生成 HTML 报告...');
  // 先生成 JSON 报告
  execSync(`oxlint --format json > "${join(cacheDir, 'report.json')}"`, { stdio: 'pipe' });
  // 再生成 HTML 报告
  execSync(`node "${join(__dirname, 'generate-lint-report.js')}"`, { stdio: 'inherit' });
}

function handleOpen() {
  console.log('🌐 打开 HTML 报告...');
  const htmlPath = join(cacheDir, 'report.html');
  if (existsSync(htmlPath)) {
    try {
      // 尝试使用不同的方式打开浏览器
      const commands = [
        `start "" "${htmlPath}"`,  // Windows 默认程序
        `cmd /c start "" "${htmlPath}"`,  // 通过 cmd 启动
        `powershell -Command "Start-Process '${htmlPath}'"`,  // PowerShell
        `rundll32 url.dll,FileProtocolHandler "${htmlPath}"`  // 系统文件协议
      ];
      
      for (const cmd of commands) {
        try {
          execSync(cmd, { stdio: 'pipe' });
          console.log('✅ HTML 报告已在浏览器中打开');
          return;
        } catch (e) {
          // 继续尝试下一个命令
        }
      }
      
      // 如果所有命令都失败，显示文件路径
      console.log('❌ 无法自动打开浏览器，请手动打开文件:');
      console.log('📁 文件路径:', htmlPath);
    } catch (error) {
      console.log('❌ 打开失败，请手动打开文件:');
      console.log('📁 文件路径:', htmlPath);
    }
  } else {
    console.log('❌ HTML 报告不存在，请先运行: pnpm run lint:html');
  }
}

function showHelp() {
  console.log('📋 可用的命令:');
  console.log('  check  - 运行代码检查');
  console.log('  fix    - 自动修复问题');
  console.log('  report - 生成 JSON 报告');
  console.log('  html   - 生成 HTML 报告');
  console.log('  open   - 打开 HTML 报告');
}

console.log('🔍 Oxlint 代码检查工具');
console.log('📁 输出目录:', cacheDir);

try {
  switch (command) {
    case 'check':
      handleCheck();
      break;
      
    case 'fix':
      handleFix();
      break;
      
    case 'report':
      handleReport();
      break;
      
    case 'html':
      handleHtml();
      break;
      
    case 'open':
      handleOpen();
      break;
      
    default:
      showHelp();
      break;
  }
} catch (error) {
  console.error('❌ 执行失败:', error.message);
  process.exit(1);
}
