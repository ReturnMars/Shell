#!/usr/bin/env node

import { readFileSync, writeFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 读取 JSON 报告
const reportPath = join(__dirname, '..', '.cache', 'oxlint', 'report.json');
const reportData = JSON.parse(readFileSync(reportPath, 'utf8'));
const report = reportData.diagnostics || reportData;

// 生成 HTML 报告
function generateHTMLReport(data) {
  const errors = data.filter(item => item.severity === 'error');
  const warnings = data.filter(item => item.severity === 'warn');
  
  const html = `
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Oxlint 代码检查报告</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }
        .header h1 {
            margin: 0;
            font-size: 2.5em;
            font-weight: 300;
        }
        .stats {
            display: flex;
            justify-content: center;
            gap: 30px;
            margin-top: 20px;
        }
        .stat {
            text-align: center;
        }
        .stat-number {
            font-size: 2em;
            font-weight: bold;
            display: block;
        }
        .stat-label {
            font-size: 0.9em;
            opacity: 0.8;
        }
        .error-stat { color: #ff6b6b; }
        .warning-stat { color: #ffd93d; }
        .total-stat { color: #4ecdc4; }
        
        .content {
            padding: 30px;
        }
        
        .section {
            margin-bottom: 40px;
        }
        
        .section h2 {
            color: #333;
            border-bottom: 2px solid #eee;
            padding-bottom: 10px;
            margin-bottom: 20px;
        }
        
        .file-group {
            margin-bottom: 30px;
            border: 1px solid #eee;
            border-radius: 8px;
            overflow: hidden;
        }
        
        .file-header {
            background: #f8f9fa;
            padding: 15px 20px;
            border-bottom: 1px solid #eee;
            font-weight: bold;
            color: #495057;
        }
        
        .issue {
            padding: 15px 20px;
            border-bottom: 1px solid #f0f0f0;
            display: flex;
            align-items: flex-start;
            gap: 15px;
        }
        
        .issue:last-child {
            border-bottom: none;
        }
        
        .issue-icon {
            width: 20px;
            height: 20px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
            font-weight: bold;
            flex-shrink: 0;
            margin-top: 2px;
        }
        
        .error-icon {
            background: #ff6b6b;
            color: white;
        }
        
        .warning-icon {
            background: #ffd93d;
            color: #333;
        }
        
        .issue-content {
            flex: 1;
        }
        
        .issue-title {
            font-weight: bold;
            margin-bottom: 5px;
            color: #333;
        }
        
        .issue-message {
            color: #666;
            margin-bottom: 8px;
        }
        
        .issue-location {
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 0.9em;
            color: #888;
            background: #f8f9fa;
            padding: 5px 8px;
            border-radius: 4px;
            display: inline-block;
        }
        
        .rule-name {
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            background: #e9ecef;
            padding: 2px 6px;
            border-radius: 3px;
            font-size: 0.8em;
            color: #495057;
        }
        
        .no-issues {
            text-align: center;
            color: #28a745;
            font-size: 1.2em;
            padding: 40px;
        }
        
        .timestamp {
            text-align: center;
            color: #888;
            font-size: 0.9em;
            margin-top: 30px;
            padding-top: 20px;
            border-top: 1px solid #eee;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔍 Oxlint 代码检查报告</h1>
            <div class="stats">
                <div class="stat">
                    <span class="stat-number error-stat">${errors.length}</span>
                    <span class="stat-label">错误</span>
                </div>
                <div class="stat">
                    <span class="stat-number warning-stat">${warnings.length}</span>
                    <span class="stat-label">警告</span>
                </div>
                <div class="stat">
                    <span class="stat-number total-stat">${data.length}</span>
                    <span class="stat-label">总计</span>
                </div>
            </div>
        </div>
        
        <div class="content">
            ${errors.length > 0 ? `
            <div class="section">
                <h2>❌ 错误 (${errors.length})</h2>
                ${generateFileGroups(errors)}
            </div>
            ` : ''}
            
            ${warnings.length > 0 ? `
            <div class="section">
                <h2>⚠️ 警告 (${warnings.length})</h2>
                ${generateFileGroups(warnings)}
            </div>
            ` : ''}
            
            ${data.length === 0 ? `
            <div class="no-issues">
                🎉 太棒了！没有发现任何代码问题！
            </div>
            ` : ''}
        </div>
        
        <div class="timestamp">
            报告生成时间: ${new Date().toLocaleString('zh-CN')}
        </div>
    </div>
</body>
</html>`;

  return html;
}

function generateFileGroups(issues) {
  const fileGroups = {};
  
  issues.forEach(issue => {
    const filename = issue.filename;
    if (!fileGroups[filename]) {
      fileGroups[filename] = [];
    }
    fileGroups[filename].push(issue);
  });
  
  return Object.entries(fileGroups)
    .map(([file, fileIssues]) => `
      <div class="file-group">
        <div class="file-header">${file}</div>
        ${fileIssues.map(issue => `
          <div class="issue">
            <div class="issue-icon ${issue.severity === 'error' ? 'error-icon' : 'warning-icon'}">
              ${issue.severity === 'error' ? 'E' : 'W'}
            </div>
            <div class="issue-content">
              <div class="issue-title">
                <span class="rule-name">${issue.code}</span>
                ${issue.message}
              </div>
              <div class="issue-message">${issue.help || ''}</div>
              <div class="issue-location">
                行 ${issue.labels?.[0]?.span?.line || 'N/A'}:${issue.labels?.[0]?.span?.column || 'N/A'}
              </div>
            </div>
          </div>
        `).join('')}
      </div>
    `).join('');
}

// 生成 HTML 报告
const htmlReport = generateHTMLReport(report);
const htmlPath = join(__dirname, '..', '.cache', 'oxlint', 'report.html');
writeFileSync(htmlPath, htmlReport, 'utf8');

console.log('✅ HTML 报告已生成:', htmlPath);
console.log(`📊 总计: ${report.length} 个问题 (${report.filter(item => item.severity === 'error').length} 个错误, ${report.filter(item => item.severity === 'warn').length} 个警告)`);
