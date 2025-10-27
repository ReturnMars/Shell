// SSH命令执行模块
use crate::models::{ConnectionConfig, CommandOptions};
use ssh2::Channel;
use std::io::{Read, Write};
use tokio::time::Duration;

pub struct SshCommandExecutor;

impl SshCommandExecutor {
    /// 执行SSH命令
    pub async fn execute_command(
        shell_channel: &mut Channel,
        config: &ConnectionConfig,
        command: &str,
    ) -> Result<String, String> {
        Self::execute_command_with_options(shell_channel, config, command, CommandOptions::default()).await
    }

    /// 执行SSH命令 - 带选项的完整版本
    pub async fn execute_command_with_options(
        shell_channel: &mut Channel,
        config: &ConnectionConfig,
        command: &str,
        options: CommandOptions,
    ) -> Result<String, String> {
        if options.debug_output {
            println!("=== execute_command_with_options 开始 ===");
            println!("command: {:?}", command);
            println!("options: {:?}", options);
        }

        // 发送命令
        let command_to_send = if command.ends_with('\n') || command.ends_with('\r') {
            command.to_string()
        } else {
            format!("{}\n", command)
        };

        shell_channel
            .write_all(command_to_send.as_bytes())
            .map_err(|e| format!("发送命令失败: {}", e))?;
        shell_channel
            .flush()
            .map_err(|e| format!("刷新缓冲区失败: {}", e))?;

        // 智能读取输出
        Self::read_command_output(shell_channel, config, &options).await
    }

    /// 智能读取命令输出
    async fn read_command_output(
        shell_channel: &mut Channel,
        config: &ConnectionConfig,
        options: &CommandOptions,
    ) -> Result<String, String> {
        let mut output = String::new();
        let mut buffer = [0u8; 4096];

        let prompt_patterns = options.custom_prompts.as_ref()
            .unwrap_or(&config.prompt_config.patterns);

        let max_wait_time = options.timeout.unwrap_or(config.prompt_config.max_wait_time);
        let max_empty_reads = config.prompt_config.max_empty_reads;

        let start_time = std::time::Instant::now();
        let mut empty_reads = 0;
        let mut last_data_time = std::time::Instant::now();

        loop {
            // 检查超时
            if start_time.elapsed().as_millis() as u64 > max_wait_time {
                if options.debug_output {
                    println!("命令执行超时，停止读取");
                }
                break;
            }

            // 检查是否等待提示符
            if !options.wait_for_prompt {
                // 不等待提示符，只读取一次
                match shell_channel.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        let chunk = String::from_utf8_lossy(&buffer[..n]);
                        output.push_str(&chunk);
                        if options.debug_output {
                            println!("读取到{}字节: {:?}", n, chunk);
                        }
                        break;
                    }
                    Err(e) => {
                        if options.debug_output {
                            println!("读取失败: {:?}", e);
                        }
                        if e.to_string().contains("Failure while draining incoming flow") {
                            if options.debug_output {
                                println!("遇到draining flow错误，尝试继续读取");
                            }
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            continue;
                        }
                        break;
                    }
                }
            } else {
                // 等待提示符模式
                match shell_channel.read(&mut buffer) {
                    Ok(0) => {
                        empty_reads += 1;
                        if options.debug_output {
                            println!("读取到0字节，连续空读取: {}", empty_reads);
                        }

                        if empty_reads >= max_empty_reads {
                            if options.debug_output {
                                println!("连续{}次空读取，可能命令已完成", max_empty_reads);
                            }
                            break;
                        }

                        if last_data_time.elapsed().as_millis() > 1000 {
                            if options.debug_output {
                                println!("长时间无数据，可能命令已完成");
                            }
                            break;
                        }

                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    Ok(n) => {
                        empty_reads = 0;
                        last_data_time = std::time::Instant::now();

                        let chunk = String::from_utf8_lossy(&buffer[..n]);
                        output.push_str(&chunk);

                        if options.debug_output {
                            println!("读取到{}字节: {:?}", n, chunk);
                        }

                        // 检查是否包含提示符
                        if Self::detect_prompt(&chunk, prompt_patterns, &config.prompt_config) {
                            if options.debug_output {
                                println!("检测到提示符，命令执行完成");
                            }
                            break;
                        }

                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                    Err(e) => {
                        if options.debug_output {
                            println!("读取失败: {:?}", e);
                        }
                        if e.to_string().contains("Failure while draining incoming flow") {
                            if options.debug_output {
                                println!("遇到draining flow错误，尝试继续读取");
                            }
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            continue;
                        }
                        break;
                    }
                }
            }
        }

        Ok(output)
    }

    /// 检测提示符
    fn detect_prompt(
        chunk: &str,
        patterns: &[String],
        config: &crate::models::PromptConfig,
    ) -> bool {
        if !config.smart_detection {
            return patterns.iter().any(|pattern| chunk.contains(pattern));
        }

        for pattern in patterns {
            if chunk.contains(pattern) {
                let lines: Vec<&str> = chunk.split('\n').collect();
                if let Some(last_line) = lines.last() {
                    if last_line.trim_end().ends_with(pattern.trim_end()) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
