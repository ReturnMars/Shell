#!/usr/bin/env node

import { execSync } from 'child_process';
import { existsSync, mkdirSync, rmSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 获取命令行参数
const args = process.argv.slice(2);
const command = args[0] || 'check';

// 根据命令类型处理目录
const cacheDir = join(__dirname, '..', '.cache', 'oxlint');
if (command !== 'open') {
  if (existsSync(cacheDir)) {
    rmSync(cacheDir, { recursive: true, force: true });
  }
  mkdirSync(cacheDir, { recursive: true });
} else {
  if (!existsSync(cacheDir)) {
    mkdirSync(cacheDir, { recursive: true });
  }
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
  const output = execSync('oxlint --format json', { encoding: 'utf-8' });
  const reportPath = join(cacheDir, 'report.json');
  writeFileSync(reportPath, output, 'utf8');
  console.log('✅ JSON 报告已生成:', reportPath);
}

function handleHtml() {
  console.log('📊 生成 HTML 报告...');
  // 先生成 JSON 报告（oxlint 有错误时会返回非零退出码，但输出依然有用）
  let output;
  try {
    output = execSync('oxlint --format json', { encoding: 'utf-8' });
  } catch (error) {
    // 即使有错误，也尝试获取输出
    output = error.stdout || '';
  }
  
  const reportPath = join(cacheDir, 'report.json');
  writeFileSync(reportPath, output, 'utf8');
  
  // 再生成 HTML 报告
  try {
    execSync(`node "${join(__dirname, 'generate-lint-report.js')}"`, { stdio: 'inherit' });
  } catch (error) {
    console.error('❌ 生成 HTML 报告失败:', error.message);
    throw error;
  }
}

function handleOpen() {
  console.log('🌐 打开 HTML 报告...');
  const htmlPath = join(cacheDir, 'report.html');
  if (existsSync(htmlPath)) {
    try {
      // 尝试使用不同的方式打开浏览器
      const commands = [
        `start "" "${htmlPath}"`,
        `cmd /c start "" "${htmlPath}"`,
        `powershell -Command "Start-Process '${htmlPath}'"`,
        `rundll32 url.dll,FileProtocolHandler "${htmlPath}"`
      ];
      
      for (const cmd of commands) {
        try {
          execSync(cmd, { stdio: 'pipe' });
          console.log('✅ HTML 报告已在浏览器中打开');
          return;
        } catch {
          // 继续尝试下一个命令
        }
      }
      
      // 如果所有命令都失败，显示文件路径
      console.log('❌ 无法自动打开浏览器，请手动打开文件:');
      console.log('📁 文件路径:', htmlPath);
    } catch {
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
  if (error.stdout) console.error('输出:', error.stdout.toString());
  if (error.stderr) console.error('错误:', error.stderr.toString());
  process.exit(1);
}
