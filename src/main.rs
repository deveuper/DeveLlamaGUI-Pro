use eframe::egui;
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::io::Read;
use std::thread;

type SharedLogs = Arc<Mutex<Vec<String>>>;

/// 从子进程的stdout/stderr流中读取数据，写入共享日志
/// 使用原始字节读取 + lossy UTF-8解码，避免跳过非UTF-8行
fn read_stream_to_logs<R: Read>(stream: R, shared_logs: SharedLogs) {
    let mut buf = [0u8; 2048];
    let mut line_buf = Vec::with_capacity(2048);
    let mut reader = std::io::BufReader::new(stream);
    loop {
        match reader.read(&mut buf) {
            Ok(0) => break, // EOF - 进程已关闭输出
            Ok(n) => {
                line_buf.extend_from_slice(&buf[..n]);
                // 处理所有完整行
                while let Some(pos) = line_buf.iter().position(|&b| b == b'\n') {
                    let line: Vec<u8> = line_buf.drain(..=pos).collect();
                    let text = String::from_utf8_lossy(&line);
                    let text = text.trim_end_matches('\r').trim_end_matches('\n');
                    if !text.is_empty() {
                        // 同时println到终端（方便调试）和写入共享日志
                        println!("[LLAMA] {}", text);
                        if let Ok(mut logs) = shared_logs.lock() {
                            logs.push(text.to_string());
                            if logs.len() > 200 {
                                logs.drain(0..50);
                            }
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }
    // 处理缓冲区中剩余的不完整行
    if !line_buf.is_empty() {
        let text = String::from_utf8_lossy(&line_buf);
        let text = text.trim_end_matches('\r');
        if !text.is_empty() {
            println!("[LLAMA] {}", text);
            if let Ok(mut logs) = shared_logs.lock() {
                logs.push(text.to_string());
                if logs.len() > 200 {
                    logs.drain(0..50);
                }
            }
        }
    }
}

// ==================== 语言系统 ====================

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
enum Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
    Japanese,
    Korean,
}

impl Default for Language { fn default() -> Self { Language::English } }

impl Language {
    fn name(&self) -> &'static str {
        match self {
            Language::English => "language : English",
            Language::ChineseSimplified => "语言：中文-简体",
            Language::ChineseTraditional => "語言：中文-繁體",
            Language::Japanese => "言語：日本語",
            Language::Korean => "언어: 한국어",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::ChineseSimplified => "中文-简体",
            Language::ChineseTraditional => "中文-繁體",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
        }
    }

    fn all() -> [Language; 5] {
        [Language::English, Language::ChineseSimplified, Language::ChineseTraditional, Language::Japanese, Language::Korean]
    }
}

// ==================== 翻译系统 ====================

fn t(lang: Language, key: &str) -> String {
    match lang {
        Language::English => match key {
            "app_title" => "DeveLlamaGUI Pro".to_string(),
            "model" => "Model:".to_string(),
            "server" => "Server:".to_string(),
            "mmproj" => "MMProj:".to_string(),
            "browse" => "Browse".to_string(),
            "clear" => "Clear".to_string(),
            "host" => "Host:".to_string(),
            "port" => "Port:".to_string(),
            "gpu_layers" => "GPU Layers:".to_string(),
            "max" => "Max".to_string(),
            "ctx_size" => "Context Size:".to_string(),
            "parallel_slots" => "Parallel Slots:".to_string(),
            "kv_cache" => "KV Cache:".to_string(),
            "threads" => "Threads:".to_string(),
            "batch_threads" => "Batch Threads:".to_string(),
            "batch_size" => "Batch Size:".to_string(),
            "micro_batch" => "Micro Batch:".to_string(),
            "flash_attn" => "Flash Attention".to_string(),
            "warmup" => "Warmup".to_string(),
            "cont_batching" => "Continuous Batching".to_string(),
            "verbose" => "Verbose Logging".to_string(),
            "start" => "START".to_string(),
            "stop" => "STOP".to_string(),
            "open_webui" => "Open WebUI".to_string(),
            "preset" => "Preset".to_string(),
            "save" => "Save".to_string(),
            "apply" => "Apply".to_string(),
            "select_preset" => "Select preset...".to_string(),
            "temperature_sampling" => "Temperature & Sampling".to_string(),
            "temperature" => "Temperature:".to_string(),
            "top_k" => "Top K:".to_string(),
            "top_p" => "Top P:".to_string(),
            "min_p" => "Min P:".to_string(),
            "repetition_control" => "Repetition Control".to_string(),
            "repeat_penalty" => "Repeat Penalty:".to_string(),
            "check_range" => "Check Range:".to_string(),
            "presence_penalty" => "Presence Penalty:".to_string(),
            "frequency_penalty" => "Frequency Penalty:".to_string(),
            "mirostat" => "Mirostat".to_string(),
            "disabled" => "Disabled".to_string(),
            "mirostat_v1" => "Mirostat 1.0".to_string(),
            "mirostat_v2" => "Mirostat 2.0".to_string(),
            "tau" => "Tau:".to_string(),
            "eta" => "Eta:".to_string(),
            "other" => "Other".to_string(),
            "max_tokens" => "Max Tokens:".to_string(),
            "seed" => "Seed:".to_string(),
            "apply_params" => "Apply Parameters".to_string(),
            "modified_click_apply" => "Modified - click Apply to save".to_string(),
            "params_saved" => "Parameters saved".to_string(),
            "runtime_params_updated" => "Runtime parameters updated".to_string(),
            "console" => "Console".to_string(),
            "cmd_preview" => "Command Preview".to_string(),
            "quick_cmds" => "Quick Commands".to_string(),
            "copy" => "Copy".to_string(),
            "refresh" => "Refresh".to_string(),
            "run" => "Run".to_string(),
            "more_cmds" => "More Commands...".to_string(),
            "save_preset" => "Save Preset".to_string(),
            "preset_name" => "Preset Name:".to_string(),
            "description" => "Description:".to_string(),
            "cancel" => "Cancel".to_string(),
            "all_cmds" => "All Commands".to_string(),
            "close" => "Close".to_string(),
            "model_server" => "Model & Server".to_string(),
            "network" => "Network".to_string(),
            "gpu_memory" => "GPU & Memory".to_string(),
            "performance" => "Performance".to_string(),
            "options" => "Options".to_string(),
            "status_running" => "Running".to_string(),
            "status_stopped" => "Stopped".to_string(),
            "server_started" => "Server started".to_string(),
            "server_stopped" => "Server stopped".to_string(),
            "server_crashed" => "Server process ended unexpectedly".to_string(),
            "preset_applied" => "Applied:".to_string(),
            "preset_saved" => "Preset saved".to_string(),
            "console_cleared" => "Console cleared".to_string(),
            "unknown_cmd" => "Unknown command:".to_string(),
            "theme" => "Theme".to_string(),
            "api_url" => "API URL".to_string(),
            "will_apply_next" => "Runtime params will be applied on next API call".to_string(),
            "launch_need_restart" => "Launch params require restart to take effect".to_string(),
            "runtime_per_request" => "Runtime params apply per API request".to_string(),
            "force_overwrite" => "Force Overwrite".to_string(),
            "restore_default" => "Restore Default".to_string(),
            "delete_preset" => "Delete".to_string(),
            "confirm_delete" => "Confirm delete?".to_string(),
            "preset_restored" => "Preset restored to default".to_string(),
            "preset_overwritten" => "Preset overwritten".to_string(),
            "preset_deleted" => "Preset deleted".to_string(),
            "cannot_delete_last" => "Cannot delete the last preset".to_string(),
            "ctx_per_seq_warn" => "ctx per seq = total ÷ parallel".to_string(),
            "server_start_cmd" => "Start command:".to_string(),
            _ => key.to_string(),
        },
        Language::ChineseSimplified => match key {
            "app_title" => "DeveLlamaGUI Pro".to_string(),
            "model" => "模型:".to_string(),
            "server" => "服务器:".to_string(),
            "mmproj" => "MMProj:".to_string(),
            "browse" => "浏览".to_string(),
            "clear" => "清除".to_string(),
            "host" => "主机:".to_string(),
            "port" => "端口:".to_string(),
            "gpu_layers" => "GPU层数:".to_string(),
            "max" => "最大".to_string(),
            "ctx_size" => "上下文大小:".to_string(),
            "parallel_slots" => "并行槽位:".to_string(),
            "kv_cache" => "KV缓存:".to_string(),
            "threads" => "线程数:".to_string(),
            "batch_threads" => "批处理线程:".to_string(),
            "batch_size" => "批处理大小:".to_string(),
            "micro_batch" => "微批处理:".to_string(),
            "flash_attn" => "Flash Attention".to_string(),
            "warmup" => "预热".to_string(),
            "cont_batching" => "连续批处理".to_string(),
            "verbose" => "详细日志".to_string(),
            "start" => "启动".to_string(),
            "stop" => "停止".to_string(),
            "open_webui" => "打开WebUI".to_string(),
            "preset" => "预设".to_string(),
            "save" => "保存".to_string(),
            "apply" => "应用".to_string(),
            "select_preset" => "选择预设...".to_string(),
            "temperature_sampling" => "温度与采样".to_string(),
            "temperature" => "温度:".to_string(),
            "top_k" => "Top K:".to_string(),
            "top_p" => "Top P:".to_string(),
            "min_p" => "Min P:".to_string(),
            "repetition_control" => "重复控制".to_string(),
            "repeat_penalty" => "重复惩罚:".to_string(),
            "check_range" => "检查范围:".to_string(),
            "presence_penalty" => "存在惩罚:".to_string(),
            "frequency_penalty" => "频率惩罚:".to_string(),
            "mirostat" => "Mirostat".to_string(),
            "disabled" => "禁用".to_string(),
            "mirostat_v1" => "Mirostat 1.0".to_string(),
            "mirostat_v2" => "Mirostat 2.0".to_string(),
            "tau" => "Tau:".to_string(),
            "eta" => "Eta:".to_string(),
            "other" => "其他".to_string(),
            "max_tokens" => "最大令牌数:".to_string(),
            "seed" => "随机种子:".to_string(),
            "apply_params" => "应用参数".to_string(),
            "modified_click_apply" => "已修改 - 点击应用保存".to_string(),
            "params_saved" => "参数已保存".to_string(),
            "runtime_params_updated" => "运行时参数已更新".to_string(),
            "console" => "控制台".to_string(),
            "cmd_preview" => "命令预览".to_string(),
            "quick_cmds" => "快速命令".to_string(),
            "copy" => "复制".to_string(),
            "refresh" => "刷新".to_string(),
            "run" => "运行".to_string(),
            "more_cmds" => "更多命令...".to_string(),
            "save_preset" => "保存预设".to_string(),
            "preset_name" => "预设名称:".to_string(),
            "description" => "描述:".to_string(),
            "cancel" => "取消".to_string(),
            "all_cmds" => "所有命令".to_string(),
            "close" => "关闭".to_string(),
            "model_server" => "模型与服务器".to_string(),
            "network" => "网络".to_string(),
            "gpu_memory" => "GPU与内存".to_string(),
            "performance" => "性能".to_string(),
            "options" => "选项".to_string(),
            "status_running" => "运行中".to_string(),
            "status_stopped" => "已停止".to_string(),
            "server_started" => "服务器已启动".to_string(),
            "server_stopped" => "服务器已停止".to_string(),
            "server_crashed" => "服务器异常终止".to_string(),
            "preset_applied" => "已应用:".to_string(),
            "preset_saved" => "预设已保存".to_string(),
            "console_cleared" => "控制台已清空".to_string(),
            "unknown_cmd" => "未知命令:".to_string(),
            "theme" => "主题".to_string(),
            "api_url" => "API地址".to_string(),
            "will_apply_next" => "运行时参数将在每次API请求时应用".to_string(),
            "launch_need_restart" => "启动参数需重启服务后生效".to_string(),
            "runtime_per_request" => "运行时参数在每次API请求时传入".to_string(),
            "force_overwrite" => "强制覆盖".to_string(),
            "restore_default" => "还原默认".to_string(),
            "delete_preset" => "删除".to_string(),
            "confirm_delete" => "确认删除？".to_string(),
            "preset_restored" => "预设已还原为默认值".to_string(),
            "preset_overwritten" => "预设已被强制覆盖".to_string(),
            "preset_deleted" => "预设已删除".to_string(),
            "cannot_delete_last" => "无法删除最后一个预设".to_string(),
            "ctx_per_seq_warn" => "每序列上下文 = 总量 ÷ 并行数".to_string(),
            "server_start_cmd" => "启动命令:".to_string(),
            _ => key.to_string(),
        },
        Language::ChineseTraditional => match key {
            "app_title" => "DeveLlamaGUI Pro".to_string(),
            "model" => "模型:".to_string(),
            "server" => "伺服器:".to_string(),
            "mmproj" => "MMProj:".to_string(),
            "browse" => "瀏覽".to_string(),
            "clear" => "清除".to_string(),
            "host" => "主機:".to_string(),
            "port" => "埠號:".to_string(),
            "gpu_layers" => "GPU層數:".to_string(),
            "max" => "最大".to_string(),
            "ctx_size" => "上下文大小:".to_string(),
            "parallel_slots" => "並行槽位:".to_string(),
            "kv_cache" => "KV快取:".to_string(),
            "threads" => "執行緒數:".to_string(),
            "batch_threads" => "批次執行緒:".to_string(),
            "batch_size" => "批次大小:".to_string(),
            "micro_batch" => "微批次:".to_string(),
            "flash_attn" => "Flash Attention".to_string(),
            "warmup" => "預熱".to_string(),
            "cont_batching" => "連續批次處理".to_string(),
            "verbose" => "詳細日誌".to_string(),
            "start" => "啟動".to_string(),
            "stop" => "停止".to_string(),
            "open_webui" => "開啟WebUI".to_string(),
            "preset" => "預設".to_string(),
            "save" => "儲存".to_string(),
            "apply" => "套用".to_string(),
            "select_preset" => "選擇預設...".to_string(),
            "temperature_sampling" => "溫度與取樣".to_string(),
            "temperature" => "溫度:".to_string(),
            "top_k" => "Top K:".to_string(),
            "top_p" => "Top P:".to_string(),
            "min_p" => "Min P:".to_string(),
            "repetition_control" => "重複控制".to_string(),
            "repeat_penalty" => "重複懲罰:".to_string(),
            "check_range" => "檢查範圍:".to_string(),
            "presence_penalty" => "存在懲罰:".to_string(),
            "frequency_penalty" => "頻率懲罰:".to_string(),
            "mirostat" => "Mirostat".to_string(),
            "disabled" => "禁用".to_string(),
            "mirostat_v1" => "Mirostat 1.0".to_string(),
            "mirostat_v2" => "Mirostat 2.0".to_string(),
            "tau" => "Tau:".to_string(),
            "eta" => "Eta:".to_string(),
            "other" => "其他".to_string(),
            "max_tokens" => "最大權杖數:".to_string(),
            "seed" => "隨機種子:".to_string(),
            "apply_params" => "套用參數".to_string(),
            "modified_click_apply" => "已修改 - 點擊套用儲存".to_string(),
            "params_saved" => "參數已儲存".to_string(),
            "runtime_params_updated" => "執行時參數已更新".to_string(),
            "console" => "控制台".to_string(),
            "cmd_preview" => "命令預覽".to_string(),
            "quick_cmds" => "快速命令".to_string(),
            "copy" => "複製".to_string(),
            "refresh" => "重新整理".to_string(),
            "run" => "執行".to_string(),
            "more_cmds" => "更多命令...".to_string(),
            "save_preset" => "儲存預設".to_string(),
            "preset_name" => "預設名稱:".to_string(),
            "description" => "描述:".to_string(),
            "cancel" => "取消".to_string(),
            "all_cmds" => "所有命令".to_string(),
            "close" => "閉".to_string(),
            "model_server" => "模型與伺服器".to_string(),
            "network" => "網路".to_string(),
            "gpu_memory" => "GPU與記憶體".to_string(),
            "performance" => "效能".to_string(),
            "options" => "選項".to_string(),
            "status_running" => "執行中".to_string(),
            "status_stopped" => "已停止".to_string(),
            "server_started" => "伺服器已啟動".to_string(),
            "server_stopped" => "伺服器已停止".to_string(),
            "server_crashed" => "伺服器異常終止".to_string(),
            "preset_applied" => "已套用:".to_string(),
            "preset_saved" => "預設已儲存".to_string(),
            "console_cleared" => "控制台已清空".to_string(),
            "unknown_cmd" => "未知命令:".to_string(),
            "theme" => "主題".to_string(),
            "api_url" => "API位址".to_string(),
            "will_apply_next" => "執行時參數將在每次API請求時套用".to_string(),
            "launch_need_restart" => "啟動參數需重啟服務後生效".to_string(),
            "runtime_per_request" => "執行時參數在每次API請求時傳入".to_string(),
            "force_overwrite" => "強制覆蓋".to_string(),
            "restore_default" => "還原預設".to_string(),
            "delete_preset" => "刪除".to_string(),
            "confirm_delete" => "確認刪除？".to_string(),
            "preset_restored" => "預設已還原為預設值".to_string(),
            "preset_overwritten" => "預設已被強制覆蓋".to_string(),
            "preset_deleted" => "預設已刪除".to_string(),
            "cannot_delete_last" => "無法刪除最後一個預設".to_string(),
            "ctx_per_seq_warn" => "每序列上下文 = 總量 ÷ 並行數".to_string(),
            "server_start_cmd" => "啟動命令:".to_string(),
            _ => key.to_string(),
        },
        Language::Japanese => match key {
            "app_title" => "DeveLlamaGUI Pro".to_string(),
            "model" => "モデル:".to_string(),
            "server" => "サーバー:".to_string(),
            "mmproj" => "MMProj:".to_string(),
            "browse" => "参照".to_string(),
            "clear" => "クリア".to_string(),
            "host" => "ホスト:".to_string(),
            "port" => "ポート:".to_string(),
            "gpu_layers" => "GPUレイヤー:".to_string(),
            "max" => "最大".to_string(),
            "ctx_size" => "コンテキストサイズ:".to_string(),
            "parallel_slots" => "並行スロット:".to_string(),
            "kv_cache" => "KVキャッシュ:".to_string(),
            "threads" => "スレッド数:".to_string(),
            "batch_threads" => "バッチスレッド:".to_string(),
            "batch_size" => "バッチサイズ:".to_string(),
            "micro_batch" => "マイクロバッチ:".to_string(),
            "flash_attn" => "Flash Attention".to_string(),
            "warmup" => "ウォームアップ".to_string(),
            "cont_batching" => "連続バッチ処理".to_string(),
            "verbose" => "詳細ログ".to_string(),
            "start" => "開始".to_string(),
            "stop" => "停止".to_string(),
            "open_webui" => "WebUIを開く".to_string(),
            "preset" => "プリセット".to_string(),
            "save" => "保存".to_string(),
            "apply" => "適用".to_string(),
            "select_preset" => "プリセットを選択...".to_string(),
            "temperature_sampling" => "温度とサンプリング".to_string(),
            "temperature" => "温度:".to_string(),
            "top_k" => "Top K:".to_string(),
            "top_p" => "Top P:".to_string(),
            "min_p" => "Min P:".to_string(),
            "repetition_control" => "繰り返し制御".to_string(),
            "repeat_penalty" => "繰り返しペナルティ:".to_string(),
            "check_range" => "チェック範囲:".to_string(),
            "presence_penalty" => "存在ペナルティ:".to_string(),
            "frequency_penalty" => "頻度ペナルティ:".to_string(),
            "mirostat" => "Mirostat".to_string(),
            "disabled" => "無効".to_string(),
            "mirostat_v1" => "Mirostat 1.0".to_string(),
            "mirostat_v2" => "Mirostat 2.0".to_string(),
            "tau" => "Tau:".to_string(),
            "eta" => "Eta:".to_string(),
            "other" => "その他".to_string(),
            "max_tokens" => "最大トークン数:".to_string(),
            "seed" => "シード値:".to_string(),
            "apply_params" => "パラメータを適用".to_string(),
            "modified_click_apply" => "変更済み - 適用をクリックして保存".to_string(),
            "params_saved" => "パラメータを保存しました".to_string(),
            "runtime_params_updated" => "実行時パラメータを更新しました".to_string(),
            "console" => "コンソール".to_string(),
            "cmd_preview" => "コマンドプレビュー".to_string(),
            "quick_cmds" => "クイックコマンド".to_string(),
            "copy" => "コピー".to_string(),
            "refresh" => "更新".to_string(),
            "run" => "実行".to_string(),
            "more_cmds" => "その他のコマンド...".to_string(),
            "save_preset" => "プリセットを保存".to_string(),
            "preset_name" => "プリセット名:".to_string(),
            "description" => "説明:".to_string(),
            "cancel" => "キャンセル".to_string(),
            "all_cmds" => "すべてのコマンド".to_string(),
            "close" => "閉じる".to_string(),
            "model_server" => "モデルとサーバー".to_string(),
            "network" => "ネットワーク".to_string(),
            "gpu_memory" => "GPUとメモリ".to_string(),
            "performance" => "パフォーマンス".to_string(),
            "options" => "オプション".to_string(),
            "status_running" => "実行中".to_string(),
            "status_stopped" => "停止".to_string(),
            "server_started" => "サーバーを開始しました".to_string(),
            "server_stopped" => "サーバーを停止しました".to_string(),
            "server_crashed" => "サーバーが異常終了しました".to_string(),
            "preset_applied" => "適用済み:".to_string(),
            "preset_saved" => "プリセットを保存しました".to_string(),
            "console_cleared" => "コンソールをクリアしました".to_string(),
            "unknown_cmd" => "不明なコマンド:".to_string(),
            "theme" => "テーマ".to_string(),
            "api_url" => "API URL".to_string(),
            "will_apply_next" => "実行時パラメータは各APIリクエスト時に適用されます".to_string(),
            "launch_need_restart" => "起動パラメータは再起動後に有効になります".to_string(),
            "runtime_per_request" => "実行時パラメータは各APIリクエストで渡されます".to_string(),
            "force_overwrite" => "強制上書き".to_string(),
            "restore_default" => "デフォルトに戻す".to_string(),
            "delete_preset" => "削除".to_string(),
            "confirm_delete" => "削除しますか？".to_string(),
            "preset_restored" => "プリセットをデフォルトに戻しました".to_string(),
            "preset_overwritten" => "プリセットを強制上書きしました".to_string(),
            "preset_deleted" => "プリセットを削除しました".to_string(),
            "cannot_delete_last" => "最後のプリセットは削除できません".to_string(),
            "ctx_per_seq_warn" => "シーケンスあたりctx = 合計 ÷ パラレル".to_string(),
            "server_start_cmd" => "起動コマンド:".to_string(),
            _ => key.to_string(),
        },
        Language::Korean => match key {
            "app_title" => "DeveLlamaGUI Pro".to_string(),
            "model" => "모델:".to_string(),
            "server" => "서버:".to_string(),
            "mmproj" => "MMProj:".to_string(),
            "browse" => "찾아보기".to_string(),
            "clear" => "지우기".to_string(),
            "host" => "호스트:".to_string(),
            "port" => "포트:".to_string(),
            "gpu_layers" => "GPU 레이어:".to_string(),
            "max" => "최대".to_string(),
            "ctx_size" => "컨텍스트 크기:".to_string(),
            "parallel_slots" => "병렬 슬롯:".to_string(),
            "kv_cache" => "KV 캐시:".to_string(),
            "threads" => "스레드 수:".to_string(),
            "batch_threads" => "배치 스레드:".to_string(),
            "batch_size" => "배치 크기:".to_string(),
            "micro_batch" => "마이크로 배치:".to_string(),
            "flash_attn" => "Flash Attention".to_string(),
            "warmup" => "웜업".to_string(),
            "cont_batching" => "연속 배치 처리".to_string(),
            "verbose" => "상세 로그".to_string(),
            "start" => "시작".to_string(),
            "stop" => "중지".to_string(),
            "open_webui" => "WebUI 열기".to_string(),
            "preset" => "프리셋".to_string(),
            "save" => "저장".to_string(),
            "apply" => "적용".to_string(),
            "select_preset" => "프리셋 선택...".to_string(),
            "temperature_sampling" => "온도 및 샘플링".to_string(),
            "temperature" => "온도:".to_string(),
            "top_k" => "Top K:".to_string(),
            "top_p" => "Top P:".to_string(),
            "min_p" => "Min P:".to_string(),
            "repetition_control" => "반복 제어".to_string(),
            "repeat_penalty" => "반복 패널티:".to_string(),
            "check_range" => "확인 범위:".to_string(),
            "presence_penalty" => "존재 패널티:".to_string(),
            "frequency_penalty" => "빈도 패널티:".to_string(),
            "mirostat" => "Mirostat".to_string(),
            "disabled" => "비활성화".to_string(),
            "mirostat_v1" => "Mirostat 1.0".to_string(),
            "mirostat_v2" => "Mirostat 2.0".to_string(),
            "tau" => "Tau:".to_string(),
            "eta" => "Eta:".to_string(),
            "other" => "기타".to_string(),
            "max_tokens" => "최대 토큰 수:".to_string(),
            "seed" => "시드 값:".to_string(),
            "apply_params" => "매개변수 적용".to_string(),
            "modified_click_apply" => "수정됨 - 저장하려면 적용을 클릭하세요".to_string(),
            "params_saved" => "매개변수가 저장되었습니다".to_string(),
            "runtime_params_updated" => "런타임 매개변수가 업데이트되었습니다".to_string(),
            "console" => "콘솔".to_string(),
            "cmd_preview" => "명령어 미리보기".to_string(),
            "quick_cmds" => "빠른 명령어".to_string(),
            "copy" => "복사".to_string(),
            "refresh" => "새로고침".to_string(),
            "run" => "실행".to_string(),
            "more_cmds" => "더 많은 명령어...".to_string(),
            "save_preset" => "프리셋 저장".to_string(),
            "preset_name" => "프리셋 이름:".to_string(),
            "description" => "설명:".to_string(),
            "cancel" => "취소".to_string(),
            "all_cmds" => "모든 명령어".to_string(),
            "close" => "닫기".to_string(),
            "model_server" => "모델 및 서버".to_string(),
            "network" => "네트워크".to_string(),
            "gpu_memory" => "GPU 및 메모리".to_string(),
            "performance" => "성능".to_string(),
            "options" => "옵션".to_string(),
            "status_running" => "실행 중".to_string(),
            "status_stopped" => "중지됨".to_string(),
            "server_started" => "서버가 시작되었습니다".to_string(),
            "server_stopped" => "서버가 중지되었습니다".to_string(),
            "server_crashed" => "서버가 예기치 않게 종료되었습니다".to_string(),
            "preset_applied" => "적용됨:".to_string(),
            "preset_saved" => "프리셋이 저장되었습니다".to_string(),
            "console_cleared" => "콘솔이 지워졌습니다".to_string(),
            "unknown_cmd" => "알 수 없는 명령어:".to_string(),
            "theme" => "테마".to_string(),
            "api_url" => "API URL".to_string(),
            "will_apply_next" => "런타임 매개변수는 각 API 요청 시 적용됩니다".to_string(),
            "launch_need_restart" => "시작 매개변수는 재시작 후 적용됩니다".to_string(),
            "runtime_per_request" => "런타임 매개변수는 각 API 요청에 전달됩니다".to_string(),
            "force_overwrite" => "강제 덮어쓰기".to_string(),
            "restore_default" => "기본값 복원".to_string(),
            "delete_preset" => "삭제".to_string(),
            "confirm_delete" => "삭제하시겠습니까?".to_string(),
            "preset_restored" => "프리셋이 기본값으로 복원되었습니다".to_string(),
            "preset_overwritten" => "프리셋이 강제 덮어쓰기되었습니다".to_string(),
            "preset_deleted" => "프리셋이 삭제되었습니다".to_string(),
            "cannot_delete_last" => "마지막 프리셋은 삭제할 수 없습니다".to_string(),
            "ctx_per_seq_warn" => "시퀀스당 ctx = 합계 ÷ 병렬".to_string(),
            "server_start_cmd" => "시작 명령:".to_string(),
            _ => key.to_string(),
        },
    }
}

// ==================== 主题系统 ====================

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
enum Theme {
    Cyberpunk, MinimalLight, SakuraPink, OceanBlue, Midnight, ForestGreen,
}

impl Default for Theme { fn default() -> Self { Theme::Cyberpunk } }

impl Theme {
    fn name(&self) -> &'static str {
        match self {
            Theme::Cyberpunk => "Cyberpunk",
            Theme::MinimalLight => "Minimal Light",
            Theme::SakuraPink => "Sakura Pink",
            Theme::OceanBlue => "Ocean Blue",
            Theme::Midnight => "Midnight",
            Theme::ForestGreen => "Forest Green",
        }
    }

    fn apply(&self, ctx: &egui::Context) {
        match self {
            Theme::Cyberpunk => {
                let mut visuals = egui::Visuals::dark();
                visuals.panel_fill = egui::Color32::from_rgb(10, 10, 20);
                visuals.window_fill = egui::Color32::from_rgb(15, 15, 30);
                visuals.extreme_bg_color = egui::Color32::from_rgb(5, 5, 15);
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(20, 20, 40);
                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 60);
                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 50, 100);
                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(70, 70, 140);
                visuals.selection.bg_fill = egui::Color32::from_rgb(180, 50, 255);
                visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 240));
                ctx.set_visuals(visuals);
            }
            Theme::MinimalLight => {
                let mut visuals = egui::Visuals::light();
                visuals.panel_fill = egui::Color32::from_rgb(250, 250, 252);
                visuals.window_fill = egui::Color32::from_rgb(255, 255, 255);
                visuals.extreme_bg_color = egui::Color32::from_rgb(245, 245, 248);
                visuals.selection.bg_fill = egui::Color32::from_rgb(0, 122, 255);
                ctx.set_visuals(visuals);
            }
            Theme::SakuraPink => {
                let mut visuals = egui::Visuals::light();
                visuals.panel_fill = egui::Color32::from_rgb(255, 245, 250);
                visuals.window_fill = egui::Color32::from_rgb(255, 250, 253);
                visuals.selection.bg_fill = egui::Color32::from_rgb(255, 105, 180);
                ctx.set_visuals(visuals);
            }
            Theme::OceanBlue => {
                let mut visuals = egui::Visuals::dark();
                visuals.panel_fill = egui::Color32::from_rgb(15, 25, 40);
                visuals.window_fill = egui::Color32::from_rgb(20, 35, 55);
                visuals.selection.bg_fill = egui::Color32::from_rgb(0, 180, 255);
                ctx.set_visuals(visuals);
            }
            Theme::Midnight => {
                let mut visuals = egui::Visuals::dark();
                visuals.panel_fill = egui::Color32::from_rgb(5, 5, 8);
                visuals.window_fill = egui::Color32::from_rgb(8, 8, 12);
                visuals.selection.bg_fill = egui::Color32::from_rgb(100, 100, 120);
                ctx.set_visuals(visuals);
            }
            Theme::ForestGreen => {
                let mut visuals = egui::Visuals::dark();
                visuals.panel_fill = egui::Color32::from_rgb(12, 25, 18);
                visuals.window_fill = egui::Color32::from_rgb(18, 35, 25);
                visuals.selection.bg_fill = egui::Color32::from_rgb(50, 200, 100);
                ctx.set_visuals(visuals);
            }
        }
    }

    fn accent_color(&self) -> egui::Color32 {
        match self {
            Theme::Cyberpunk => egui::Color32::from_rgb(180, 50, 255),
            Theme::MinimalLight => egui::Color32::from_rgb(0, 122, 255),
            Theme::SakuraPink => egui::Color32::from_rgb(255, 105, 180),
            Theme::OceanBlue => egui::Color32::from_rgb(0, 180, 255),
            Theme::Midnight => egui::Color32::from_rgb(150, 150, 170),
            Theme::ForestGreen => egui::Color32::from_rgb(50, 200, 100),
        }
    }

    fn success_color(&self) -> egui::Color32 {
        match self {
            Theme::Cyberpunk => egui::Color32::from_rgb(0, 200, 100),
            Theme::MinimalLight => egui::Color32::from_rgb(34, 197, 94),
            Theme::SakuraPink => egui::Color32::from_rgb(50, 200, 100),
            Theme::OceanBlue => egui::Color32::from_rgb(0, 255, 150),
            Theme::Midnight => egui::Color32::from_rgb(100, 220, 120),
            Theme::ForestGreen => egui::Color32::from_rgb(100, 255, 150),
        }
    }

    fn error_color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(255, 80, 80)
    }
}

// ==================== 预设配置 ====================

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Preset {
    name: String,
    description: String,
    runtime_params: RuntimeParams,
}

impl Preset {
    fn default_presets() -> Vec<Self> {
        vec![
            Preset {
                name: "General Chat".to_string(),
                description: "Balanced for everyday conversation".to_string(),
                runtime_params: RuntimeParams {
                    temperature: 0.7, top_k: 40, top_p: 0.9, min_p: 0.05,
                    repeat_penalty: 1.0, repeat_last_n: 64,
                    presence_penalty: 0.0, frequency_penalty: 0.0,
                    n_predict: -1, seed: -1, mirostat: 0,
                    mirostat_tau: 5.0, mirostat_eta: 0.1,
                },
            },
            Preset {
                name: "Code Mode".to_string(),
                description: "Optimized for coding and precise tasks".to_string(),
                runtime_params: RuntimeParams {
                    temperature: 0.6, top_k: 20, top_p: 1.0, min_p: 0.0,
                    repeat_penalty: 1.05, repeat_last_n: 128,
                    presence_penalty: 0.0, frequency_penalty: 0.0,
                    n_predict: -1, seed: -1, mirostat: 0,
                    mirostat_tau: 5.0, mirostat_eta: 0.1,
                },
            },
            Preset {
                name: "Creative Writing".to_string(),
                description: "High temperature for creative output".to_string(),
                runtime_params: RuntimeParams {
                    temperature: 0.9, top_k: 64, top_p: 0.95, min_p: 0.03,
                    repeat_penalty: 1.1, repeat_last_n: 64,
                    presence_penalty: 0.0, frequency_penalty: 0.0,
                    n_predict: -1, seed: -1, mirostat: 2,
                    mirostat_tau: 5.0, mirostat_eta: 0.1,
                },
            },
            Preset {
                name: "OpenClaw".to_string(),
                description: "Optimized for OpenClaw agent use".to_string(),
                runtime_params: RuntimeParams {
                    temperature: 0.3, top_k: 40, top_p: 0.9, min_p: 0.05,
                    repeat_penalty: 1.0, repeat_last_n: 96,
                    presence_penalty: 0.0, frequency_penalty: 0.0,
                    n_predict: 8192, seed: -1, mirostat: 0,
                    mirostat_tau: 5.0, mirostat_eta: 0.1,
                },
            },
            Preset {
                name: "Roleplay".to_string(),
                description: "For character roleplay scenarios".to_string(),
                runtime_params: RuntimeParams {
                    temperature: 0.8, top_k: 64, top_p: 0.92, min_p: 0.03,
                    repeat_penalty: 1.1, repeat_last_n: 64,
                    presence_penalty: 0.0, frequency_penalty: 0.0,
                    n_predict: -1, seed: -1, mirostat: 2,
                    mirostat_tau: 4.0, mirostat_eta: 0.15,
                },
            },
            Preset {
                name: "Math & Logic".to_string(),
                description: "Deterministic for math and reasoning".to_string(),
                runtime_params: RuntimeParams {
                    temperature: 0.1, top_k: 10, top_p: 0.8, min_p: 0.1,
                    repeat_penalty: 1.0, repeat_last_n: 128,
                    presence_penalty: 0.0, frequency_penalty: 0.0,
                    n_predict: -1, seed: -1, mirostat: 0,
                    mirostat_tau: 5.0, mirostat_eta: 0.1,
                },
            },
            Preset {
                name: "Brainstorm".to_string(),
                description: "Maximum creativity and divergence".to_string(),
                runtime_params: RuntimeParams {
                    temperature: 1.2, top_k: 100, top_p: 0.98, min_p: 0.01,
                    repeat_penalty: 1.0, repeat_last_n: 32,
                    presence_penalty: 0.0, frequency_penalty: 0.0,
                    n_predict: -1, seed: -1, mirostat: 0,
                    mirostat_tau: 5.0, mirostat_eta: 0.1,
                },
            },
        ]
    }
}

// ==================== 数据结构 ====================

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LaunchConfig {
    model_path: String,
    llama_server_path: String,
    host: String,
    port: u16,
    n_gpu_layers: i32,
    ctx_size: i32,
    threads: i32,
    threads_batch: i32,
    batch_size: i32,
    ubatch_size: i32,
    flash_attn: bool,
    kv_cache_type: String,
    verbose: bool,
    warmup: bool,
    continuous_batching: bool,
    n_parallel: i32,
    mmproj_path: String,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            model_path: r"H:\AI\models\model.gguf".to_string(),
            llama_server_path: r"H:\AI\llama.cpp\llama-server.exe".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            n_gpu_layers: 999,
            ctx_size: 4096,
            threads: 8,
            threads_batch: 8,
            batch_size: 2048,
            ubatch_size: 512,
            flash_attn: true,
            kv_cache_type: "f16".to_string(),
            verbose: false,
            warmup: true,
            continuous_batching: true,
            n_parallel: 1,
            mmproj_path: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RuntimeParams {
    temperature: f32, top_k: i32, top_p: f32, min_p: f32,
    repeat_penalty: f32, repeat_last_n: i32,
    presence_penalty: f32, frequency_penalty: f32,
    n_predict: i32, seed: i32,
    mirostat: i32, mirostat_tau: f32, mirostat_eta: f32,
}

impl Default for RuntimeParams {
    fn default() -> Self {
        Self {
            temperature: 0.6, top_k: 20, top_p: 1.0, min_p: 0.0,
            repeat_penalty: 1.05, repeat_last_n: 64,
            presence_penalty: 0.0, frequency_penalty: 0.0,
            n_predict: -1, seed: -1, mirostat: 0,
            mirostat_tau: 5.0, mirostat_eta: 0.1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AppSettings {
    theme: Theme,
    language: Language,
    presets: Vec<Preset>,
    last_preset_index: Option<usize>,
    #[serde(default)]
    default_presets_snapshot: Vec<Preset>, // 初始预设快照，用于"还原默认"
}

impl Default for AppSettings {
    fn default() -> Self {
        let presets = Preset::default_presets();
        Self {
            theme: Theme::Cyberpunk,
            language: Language::English,
            default_presets_snapshot: presets.clone(),
            presets,
            last_preset_index: None,
        }
    }
}

// ==================== 常用命令 ====================
#[derive(Clone)]
struct QuickCommand {
    name: String,
    description: String,
    command: String,
}

impl QuickCommand {
    fn default_commands(lang: Language) -> Vec<Self> {
        vec![
            QuickCommand {
                name: t(lang, "clear").to_string(),
                description: "Clear console logs".to_string(),
                command: "/clear".to_string(),
            },
            QuickCommand {
                name: "Copy API URL".to_string(),
                description: "Copy API endpoint to clipboard".to_string(),
                command: "api_url".to_string(),
            },
            QuickCommand {
                name: "Open Docs".to_string(),
                description: "Open llama.cpp documentation".to_string(),
                command: "open_docs".to_string(),
            },
        ]
    }
}

// ==================== 主应用 ====================

struct DeveLlamaGUI {
    launch_config: LaunchConfig,
    runtime_params: RuntimeParams,
    settings: AppSettings,
    server_process: Arc<Mutex<Option<Child>>>,
    logs: Vec<String>,
    shared_logs: SharedLogs,
    is_running: bool,
    status_message: String,
    api_url: String,
    cmd_preview: String,
    params_modified: bool,
    params_apply_status: String,
    show_save_preset_dialog: bool,
    new_preset_name: String,
    new_preset_description: String,
    confirm_delete_preset_idx: Option<usize>, // 正在确认删除的预设索引
    console_input: String,
    quick_commands: Vec<QuickCommand>,
    show_all_commands: bool,
    config_file_path: PathBuf,
    launch_config_dirty: bool, // 启动参数在运行期间被修改，需要重启
    server_start_time: Option<std::time::Instant>, // 服务器启动时间，用于延迟验证
    ctx_verified: bool, // ctx_size是否已通过/props验证
    ctx_mismatch_warned: bool, // 是否已警告ctx_size不匹配
}

impl DeveLlamaGUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 加载字体支持多语言
        let mut fonts = egui::FontDefinitions::default();
        
        // 主字体：微软雅黑（支持中文简繁体+日文）
        // 备用字体：Malgun Gothic（支持韩文）
        // 这样CJK语言都能正确显示
        let primary_fonts = [
            r"C:\Windows\Fonts\msyh.ttc",
            r"C:\Windows\Fonts\simsun.ttc",
            r"C:\Windows\Fonts\segoeui.ttf",
            r"C:\Windows\Fonts\arial.ttf",
        ];
        let korean_fonts = [
            r"C:\Windows\Fonts\malgun.ttf",
            r"C:\Windows\Fonts\malgunbd.ttf",
        ];
        
        // 加载主字体（中文/日文）
        for path in &primary_fonts {
            if let Ok(font_data) = std::fs::read(path) {
                let name = std::path::Path::new(path).file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("primary_font");
                fonts.font_data.insert(
                    name.to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, name.to_owned());
                break;
            }
        }
        // 加载韩文字体作为fallback（malgun.ttf也包含中文，但雅黑优先）
        for path in &korean_fonts {
            if let Ok(font_data) = std::fs::read(path) {
                let name = std::path::Path::new(path).file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("korean_font");
                fonts.font_data.insert(
                    name.to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().push(name.to_owned());
                break;
            }
        }
        
        cc.egui_ctx.set_fonts(fonts);

        let launch_config = LaunchConfig::default();
        let runtime_params = RuntimeParams::default();
        let settings = AppSettings::default();
        let api_url = format!("http://{}:{}/v1", launch_config.host, launch_config.port);
        let cmd_preview = Self::build_command(&launch_config);
        let config_file_path = Self::get_config_path();
        
        let (loaded_settings, loaded_launch, loaded_runtime) = Self::load_config(&config_file_path);
        let mut settings = loaded_settings.unwrap_or(settings);
        // 如果default_presets_snapshot为空(旧配置文件)，用当前默认预设填充
        if settings.default_presets_snapshot.is_empty() {
            settings.default_presets_snapshot = Preset::default_presets();
        }
        let launch_config = loaded_launch.unwrap_or(launch_config);
        let runtime_params = loaded_runtime.unwrap_or(runtime_params);
        
        let quick_commands = QuickCommand::default_commands(settings.language);
        
        let mut gui = Self {
            launch_config,
            runtime_params,
            settings,
            server_process: Arc::new(Mutex::new(None)),
            logs: vec!["DeveLlamaGUI Pro Ready".to_string()],
            shared_logs: Arc::new(Mutex::new(Vec::new())),
            is_running: false,
            status_message: "Stopped".to_string(),
            api_url,
            cmd_preview,
            params_modified: false,
            params_apply_status: String::new(),
            show_save_preset_dialog: false,
            new_preset_name: String::new(),
            new_preset_description: String::new(),
            confirm_delete_preset_idx: None,
            console_input: String::new(),
            quick_commands,
            show_all_commands: false,
            config_file_path,
            launch_config_dirty: false,
            server_start_time: None,
            ctx_verified: false,
            ctx_mismatch_warned: false,
        };
        gui.update_command_preview();
        gui
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("DeveLlamaGUI");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    fn load_config(path: &PathBuf) -> (Option<AppSettings>, Option<LaunchConfig>, Option<RuntimeParams>) {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                let settings = config.get("settings").and_then(|v| serde_json::from_value(v.clone()).ok());
                let launch = config.get("launch_config").and_then(|v| serde_json::from_value(v.clone()).ok());
                let runtime = config.get("runtime_params").and_then(|v| serde_json::from_value(v.clone()).ok());
                return (settings, launch, runtime);
            }
        }
        (None, None, None)
    }

    fn save_config(&self) {
        let config = serde_json::json!({
            "settings": self.settings,
            "launch_config": self.launch_config,
            "runtime_params": self.runtime_params,
        });
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            let _ = std::fs::write(&self.config_file_path, json);
        }
    }

    fn set_language(&mut self, lang: Language) {
        self.settings.language = lang;
        self.quick_commands = QuickCommand::default_commands(lang);
        self.save_config();
    }

    fn get_model_name(&self) -> String {
        std::path::Path::new(&self.launch_config.model_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown Model")
            .to_string()
    }

    fn apply_preset(&mut self, index: usize) {
        let lang = self.settings.language;
        if let Some(preset) = self.settings.presets.get(index) {
            self.runtime_params = preset.runtime_params.clone();
            self.settings.last_preset_index = Some(index);
            self.params_modified = false;
            self.params_apply_status = format!("{} {}", t(lang, "preset_applied"), preset.name);
            self.logs.push(format!("{}: {}", t(lang, "preset_applied"), preset.name));
            self.save_config();
        }
    }

    fn save_current_as_preset(&mut self) {
        let lang = self.settings.language;
        if !self.new_preset_name.is_empty() {
            let preset = Preset {
                name: self.new_preset_name.clone(),
                description: self.new_preset_description.clone(),
                runtime_params: self.runtime_params.clone(),
            };
            self.settings.presets.push(preset);
            self.save_config();
            self.show_save_preset_dialog = false;
            self.new_preset_name.clear();
            self.new_preset_description.clear();
            self.logs.push(t(lang, "preset_saved").to_string());
        }
    }

    /// 强制覆盖当前选中的预设为当前运行时参数
    fn force_overwrite_preset(&mut self) {
        let lang = self.settings.language;
        if let Some(idx) = self.settings.last_preset_index {
            // 先克隆运行时参数，避免借用冲突
            let params = self.runtime_params.clone();
            if let Some(preset) = self.settings.presets.get_mut(idx) {
                let name = preset.name.clone();
                preset.runtime_params = params;
                self.save_config();
                self.params_modified = false;
                self.params_apply_status = format!("{}: {}", t(lang, "preset_overwritten"), name);
                self.logs.push(format!("{}: {}", t(lang, "preset_overwritten"), name));
            }
        }
    }

    /// 还原当前选中的预设为默认值
    fn restore_preset_default(&mut self) {
        let lang = self.settings.language;
        if let Some(idx) = self.settings.last_preset_index {
            if let Some(default_preset) = self.settings.default_presets_snapshot.get(idx).cloned() {
                // 先取出默认参数的克隆
                let default_params = default_preset.runtime_params.clone();
                let default_name = default_preset.name.clone();
                if let Some(preset) = self.settings.presets.get_mut(idx) {
                    preset.runtime_params = default_params;
                }
                self.runtime_params = default_preset.runtime_params;
                self.params_modified = false;
                self.save_config();
                self.params_apply_status = format!("{}: {}", t(lang, "preset_restored"), default_name);
                self.logs.push(format!("{}: {}", t(lang, "preset_restored"), default_name));
            } else {
                // 非默认预设（用户自己添加的），没有默认快照，提示无法还原
                self.logs.push("Cannot restore: this is a custom preset with no default.".to_string());
            }
        }
    }

    /// 删除指定索引的预设
    fn delete_preset(&mut self, idx: usize) {
        let lang = self.settings.language;
        if self.settings.presets.len() <= 1 {
            self.logs.push(t(lang, "cannot_delete_last").to_string());
            return;
        }
        let name = self.settings.presets[idx].name.clone();
        self.settings.presets.remove(idx);
        // 修正 last_preset_index
        if let Some(current) = self.settings.last_preset_index {
            if current == idx {
                self.settings.last_preset_index = None;
            } else if current > idx {
                self.settings.last_preset_index = Some(current - 1);
            }
        }
        self.save_config();
        self.logs.push(format!("{}: {}", t(lang, "preset_deleted"), name));
    }

    fn execute_console_command(&mut self) {
        let cmd = self.console_input.trim();
        if cmd.is_empty() { return; }
        let lang = self.settings.language;
        
        self.logs.push(format!("> {}", cmd));
        
        match cmd {
            "/clear" => {
                self.logs.clear();
                self.logs.push(t(lang, "console_cleared").to_string());
            }
            "/help" => {
                self.logs.push("Commands: /clear, /help".to_string());
            }
            "api_url" => {
                self.copy_to_clipboard(&self.api_url);
                self.logs.push(format!("API URL copied: {}", self.api_url));
            }
            "open_docs" => {
                let mut cmd = Command::new("cmd");
                cmd.args(["/c", "start", "", "https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md"]);
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x08000000);
                }
                let _ = cmd.spawn();
                self.logs.push("Opening documentation...".to_string());
            }
            _ => {
                self.logs.push(format!("{} {}", t(lang, "unknown_cmd"), cmd));
            }
        }
        
        self.console_input.clear();
    }

    #[allow(dead_code)]
    fn send_quick_command(&mut self, cmd: &str) {
        self.console_input = cmd.to_string();
        self.execute_console_command();
    }

    fn build_command(config: &LaunchConfig) -> String {
        let mut cmd = format!(
            "{} ^\n  --model \"{}\" ^\n  --host {} ^\n  --port {} ^\n  --n-gpu-layers {} ^\n  --ctx-size {} ^\n  --threads {} ^\n  --threads-batch {} ^\n  --batch-size {} ^\n  --ubatch-size {}",
            config.llama_server_path, config.model_path, config.host, config.port,
            config.n_gpu_layers, config.ctx_size, config.threads, config.threads_batch,
            config.batch_size, config.ubatch_size
        );
        if config.flash_attn { cmd.push_str(" ^\n  --flash-attn on"); }
        // 始终传递KV缓存类型
        cmd.push_str(&format!(" ^\n  --cache-type-k {}", config.kv_cache_type));
        cmd.push_str(&format!(" ^\n  --cache-type-v {}", config.kv_cache_type));
        if config.verbose { cmd.push_str(" ^\n  --verbose"); }
        if !config.warmup { cmd.push_str(" ^\n  --no-warmup"); }
        if !config.continuous_batching { cmd.push_str(" ^\n  --no-cont-batching"); }
        // 始终传递parallel参数
        cmd.push_str(&format!(" ^\n  --parallel {}", config.n_parallel));
        if !config.mmproj_path.is_empty() { cmd.push_str(&format!(" ^\n  --mmproj \"{}\"", config.mmproj_path)); }
        cmd.push_str(" ^\n  --props");
        cmd
    }

    fn update_command_preview(&mut self) {
        self.cmd_preview = Self::build_command(&self.launch_config);
        self.api_url = format!("http://{}:{}/v1", self.launch_config.host, self.launch_config.port);
    }

    fn start_server(&mut self) {
        self.update_command_preview();
        let config = &self.launch_config;
        let mut cmd = Command::new(&config.llama_server_path);
        
        // Windows下设置CREATE_NO_WINDOW标志，避免弹出控制台窗口
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        
        cmd.arg("--model").arg(&config.model_path)
            .arg("--host").arg(&config.host)
            .arg("--port").arg(config.port.to_string())
            .arg("--n-gpu-layers").arg(config.n_gpu_layers.to_string())
            .arg("--ctx-size").arg(config.ctx_size.to_string())
            .arg("--threads").arg(config.threads.to_string())
            .arg("--threads-batch").arg(config.threads_batch.to_string())
            .arg("--batch-size").arg(config.batch_size.to_string())
            .arg("--ubatch-size").arg(config.ubatch_size.to_string());

        if config.flash_attn { cmd.arg("--flash-attn").arg("on"); }
        // 始终传递KV缓存类型，确保llama-server使用正确的缓存格式
        cmd.arg("--cache-type-k").arg(&config.kv_cache_type);
        cmd.arg("--cache-type-v").arg(&config.kv_cache_type);
        if config.verbose { cmd.arg("--verbose"); }
        if !config.warmup { cmd.arg("--no-warmup"); }
        if !config.continuous_batching { cmd.arg("--no-cont-batching"); }
        // 始终传递--parallel参数，让用户清楚parallel和ctx-size的关系
        cmd.arg("--parallel").arg(config.n_parallel.to_string());
        if !config.mmproj_path.is_empty() { cmd.arg("--mmproj").arg(&config.mmproj_path); }
        // 启用props API端点，允许通过GET /props查看运行时属性
        cmd.arg("--props");

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let lang = self.settings.language;
        // 记录完整启动命令到日志，方便用户验证参数
        self.logs.push(format!("{} {}", t(lang, "server_start_cmd"), self.cmd_preview.replace(" ^\n  ", " ")));
        if config.n_parallel > 1 {
            let per_seq = config.ctx_size / config.n_parallel;
            self.logs.push(format!("[INFO] ctx-size={}, parallel={}, ctx per sequence={}", 
                config.ctx_size, config.n_parallel, per_seq));
        }

        match cmd.spawn() {
            Ok(mut child) => {
                // 分别用独立线程读取stdout和stderr，避免互相阻塞
                // 使用原始字节读取 + lossy UTF-8解码，避免跳过非UTF-8行
                
                // 读取stdout的后台线程
                if let Some(out) = child.stdout.take() {
                    let shared_logs = self.shared_logs.clone();
                    thread::spawn(move || {
                        read_stream_to_logs(out, shared_logs);
                    });
                }
                
                // 读取stderr的后台线程（llama-server的info也输出到stderr）
                if let Some(err) = child.stderr.take() {
                    let shared_logs = self.shared_logs.clone();
                    thread::spawn(move || {
                        read_stream_to_logs(err, shared_logs);
                    });
                }
                
                *self.server_process.lock().unwrap() = Some(child);
                self.is_running = true;
                self.launch_config_dirty = false;
                self.server_start_time = Some(std::time::Instant::now());
                self.ctx_verified = false;
                self.ctx_mismatch_warned = false;
                self.status_message = format!("Running on port {}", config.port);
                self.logs.push(t(lang, "server_started").to_string());
                self.logs.push(format!("API: {}", self.api_url));
                self.params_apply_status = t(lang, "will_apply_next").to_string();
            }
            Err(e) => {
                self.status_message = format!("Failed: {}", e);
                self.logs.push(format!("Error: {}", e));
            }
        }
    }

    fn stop_server(&mut self) {
        if let Some(mut child) = self.server_process.lock().unwrap().take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        let lang = self.settings.language;
        self.is_running = false;
        self.launch_config_dirty = false;
        self.server_start_time = None;
        self.ctx_verified = false;
        self.ctx_mismatch_warned = false;
        self.status_message = t(lang, "status_stopped").to_string();
        self.logs.push(t(lang, "server_stopped").to_string());
        self.params_apply_status.clear();
    }

    fn apply_runtime_params(&mut self) {
        let lang = self.settings.language;
        self.params_modified = false;
        self.save_config();
        
        // llama-server的运行时参数（temperature等）不能全局修改，
        // 它们是在每次/completion或/v1/chat/completions请求时传入的。
        // 但我们可以通过GET /props验证服务器是否在运行，并读取当前默认值
        if self.is_running {
            let url = format!("http://{}:{}/props", self.launch_config.host, self.launch_config.port);
            let mut cmd = Command::new("curl");
            cmd.args(["-s", &url]);
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
            }
            
            match cmd.output() {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        self.params_apply_status = t(lang, "runtime_params_updated").to_string();
                        self.logs.push(format!("[API] Server props: {}", stdout.trim()));
                        self.logs.push("[INFO] Note: Runtime params (temp, top_k, etc.) are applied per-request via /completion or /v1/chat/completions API.".to_string());

                        // 验证ctx_size是否与设置一致
                        if let Ok(props) = serde_json::from_str::<serde_json::Value>(&stdout) {
                            if let Some(n_ctx) = props.get("n_ctx").and_then(|v| v.as_i64()) {
                                let expected = self.launch_config.ctx_size as i64;
                                if n_ctx != expected {
                                    self.logs.push(format!(
                                        "[WARN] ctx_size mismatch! Server actual: {}, GUI setting: {}. The model may not support the requested context size.",
                                        n_ctx, expected
                                    ));
                                    self.logs.push(format!(
                                        "[INFO] The actual ctx per sequence may be {} (total / parallel). Model max context may limit the value.",
                                        n_ctx
                                    ));
                                } else {
                                    self.logs.push(format!("[OK] ctx_size verified: {} matches GUI setting", n_ctx));
                                }
                            }
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        self.params_apply_status = format!("API Error: {}", stderr.trim());
                        self.logs.push(format!("[API] Error: {}", stderr.trim()));
                    }
                }
                Err(e) => {
                    self.params_apply_status = format!("Failed: {}", e);
                    self.logs.push(format!("[API] curl failed: {}", e));
                }
            }
        } else {
            self.params_apply_status = t(lang, "params_saved").to_string();
            self.logs.push(t(lang, "params_saved").to_string());
        }
    }

    fn on_launch_param_changed(&mut self) {
        self.update_command_preview();
        self.save_config();
        if self.is_running {
            self.launch_config_dirty = true;
        }
    }

    /// 启动后自动验证ctx_size是否与llama-server实际值匹配
    /// 通过GET /props获取n_ctx，与GUI设置对比
    fn auto_verify_ctx(&mut self) {
        if self.ctx_verified || !self.is_running { return; }
        
        let url = format!("http://{}:{}/props", self.launch_config.host, self.launch_config.port);
        let mut cmd = Command::new("curl");
        cmd.args(["-s", "--connect-timeout", "2", &url]);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000);
        }
        
        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Ok(props) = serde_json::from_str::<serde_json::Value>(&stdout) {
                        if let Some(n_ctx) = props.get("n_ctx").and_then(|v| v.as_i64()) {
                            let expected = self.launch_config.ctx_size as i64;
                            if n_ctx == expected {
                                self.ctx_verified = true;
                                self.logs.push(format!("[✓] ctx_size verified: {} (matches GUI setting)", n_ctx));
                            } else {
                                self.ctx_verified = true;
                                self.ctx_mismatch_warned = true;
                                self.logs.push(format!(
                                    "[⚠] ctx_size MISMATCH! Server actual: {}, GUI setting: {}",
                                    n_ctx, expected
                                ));
                                if self.launch_config.n_parallel > 1 {
                                    self.logs.push(format!(
                                        "[INFO] This may be due to --parallel {}: ctx_per_seq = {} / {} = {}",
                                        self.launch_config.n_parallel,
                                        n_ctx,
                                        self.launch_config.n_parallel,
                                        n_ctx / self.launch_config.n_parallel as i64
                                    ));
                                }
                                self.logs.push("[TIP] The model may not support the requested context size. Check llama-server logs for details.".to_string());
                            }
                        }
                    }
                }
                // 如果连接失败（服务器还没启动完成），不标记verified，下次再试
            }
            Err(_) => {
                // curl不可用或服务器还没启动，不标记verified
            }
        }
    }

    fn on_runtime_param_changed(&mut self) {
        self.params_modified = true;
        self.params_apply_status = t(self.settings.language, "modified_click_apply").to_string();
    }

    fn open_model_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("GGUF models", &["gguf"])
            .set_directory("H:\\AI\\models")
            .pick_file() {
            self.launch_config.model_path = path.to_string_lossy().to_string();
            self.update_command_preview();
            self.save_config();
        }
    }

    fn open_exe_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Executable", &["exe"])
            .set_directory("H:\\AI\\llama.cpp")
            .pick_file() {
            self.launch_config.llama_server_path = path.to_string_lossy().to_string();
            self.update_command_preview();
            self.save_config();
        }
    }

    fn open_mmproj_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("MMProj files", &["gguf"])
            .set_directory("H:\\AI\\models")
            .pick_file() {
            self.launch_config.mmproj_path = path.to_string_lossy().to_string();
            self.update_command_preview();
            self.save_config();
        }
    }

    fn open_web_ui(&self) {
        let url = format!("http://localhost:{}", self.launch_config.port);
        let mut cmd = Command::new("cmd");
        cmd.args(["/c", "start", "", &url]);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        let _ = cmd.spawn();
    }

    fn copy_to_clipboard(&self, text: &str) {
        let mut cmd = Command::new("powershell");
        cmd.args(["-command", &format!("Set-Clipboard -Value '{}'", text.replace("'", "''"))]);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        let _ = cmd.spawn();
    }
}


impl eframe::App for DeveLlamaGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 同步后台线程的日志到主日志
        if let Ok(mut shared) = self.shared_logs.lock() {
            if !shared.is_empty() {
                // 检测llama-server输出中的关键错误/警告信息
                for line in shared.iter() {
                    let line_lower = line.to_lowercase();
                    if line_lower.contains("ggml_assert") || line_lower.contains("assert") && line_lower.contains("failed") {
                        self.ctx_mismatch_warned = true;
                        self.ctx_verified = true; // 标记为已验证（虽然验证结果为失败）
                    }
                    if line_lower.contains("n_ctx_per_seq") && line_lower.contains("n_ctx_train") {
                        // llama-server提示每序列上下文小于训练上下文
                        if !self.ctx_mismatch_warned {
                            self.ctx_mismatch_warned = true;
                            self.ctx_verified = true;
                        }
                    }
                    if line_lower.contains("full capacity") && line_lower.contains("not be utilized") {
                        // "the full capacity of the model will not be utilized"
                        if !self.ctx_mismatch_warned {
                            self.ctx_mismatch_warned = true;
                            self.ctx_verified = true;
                        }
                    }
                }
                self.logs.append(&mut shared);
                // 限制主日志数量
                if self.logs.len() > 200 {
                    self.logs.drain(0..50);
                }
            }
        }
        
        self.settings.theme.apply(ctx);
        let accent = self.settings.theme.accent_color();
        let success = self.settings.theme.success_color();
        let error = self.settings.theme.error_color();
        let lang = self.settings.language;

        // 进程运行时用较低频率刷新以检测退出状态
        if self.is_running {
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
            
            // 启动后3秒自动验证ctx_size是否与llama-server实际值匹配
            if !self.ctx_verified {
                if let Some(start_time) = self.server_start_time {
                    if start_time.elapsed() >= std::time::Duration::from_secs(3) {
                        self.auto_verify_ctx();
                    }
                }
            }
        }

        // 全局字体变大
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::new(22.0, egui::FontFamily::Proportional));
        style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(15.0, egui::FontFamily::Proportional));
        style.text_styles.insert(egui::TextStyle::Monospace, egui::FontId::new(13.0, egui::FontFamily::Monospace));
        style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(15.0, egui::FontFamily::Proportional));
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        ctx.set_style(style);

        // ========== 顶部标题栏（标题 + 启动/停止/打开WebUI按钮 + 状态信息） ==========
        egui::TopBottomPanel::top("title_bar").exact_height(90.0).show(ctx, |ui| {
            // 上边留白，不顶上边栏
            ui.add_space(12.0);
            
            // 第一行：标题 + 状态
            ui.horizontal(|ui| {
                ui.add_space(14.0);
                ui.label(egui::RichText::new(t(lang, "app_title")).size(24.0).color(accent).strong());
                ui.separator();
                
                let model_name = self.get_model_name();
                ui.label(egui::RichText::new(format!("{} {}", t(lang, "model"), model_name)).size(14.0));
                ui.separator();
                
                let (status_text, status_color) = if self.is_running {
                    (t(lang, "status_running"), success)
                } else {
                    (t(lang, "status_stopped"), error)
                };
                ui.label(egui::RichText::new(status_text).size(14.0).color(status_color));
                
                // 如果运行中且启动参数被修改，显示警告
                if self.is_running && self.launch_config_dirty {
                    ui.separator();
                    ui.label(egui::RichText::new(format!("⚠ {}", t(lang, "launch_need_restart"))).size(13.0).color(egui::Color32::YELLOW));
                }
                
                // 如果ctx_size验证不匹配，显示红色警告
                if self.is_running && self.ctx_mismatch_warned {
                    ui.separator();
                    ui.label(egui::RichText::new("⚠ ctx_size mismatch!").size(13.0).color(error));
                }
                
                // 如果ctx_size验证通过，显示绿色标记
                if self.is_running && self.ctx_verified && !self.ctx_mismatch_warned {
                    ui.separator();
                    ui.label(egui::RichText::new("✓ ctx OK").size(13.0).color(success));
                }
                
                ui.separator();
                ui.monospace(&self.api_url);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(14.0);
                    
                    // 语言切换菜单
                    ui.menu_button(self.settings.language.name(), |ui| {
                        for l in Language::all() {
                            if ui.selectable_label(self.settings.language == l, l.display_name()).clicked() {
                                self.set_language(l);
                            }
                        }
                    });
                    ui.separator();
                    
                    ui.menu_button(t(lang, "theme"), |ui| {
                        for theme in [Theme::Cyberpunk, Theme::MinimalLight, Theme::SakuraPink, 
                                      Theme::OceanBlue, Theme::Midnight, Theme::ForestGreen] {
                            if ui.selectable_label(self.settings.theme == theme, theme.name()).clicked() {
                                self.settings.theme = theme;
                                self.save_config();
                            }
                        }
                    });
                });
            });
            
            ui.add_space(4.0);
            
            // 第二行：启动/停止 + 打开WebUI + 复制命令 按钮
            ui.horizontal(|ui| {
                ui.add_space(14.0);
                ui.spacing_mut().button_padding = egui::vec2(18.0, 8.0);
                
                if !self.is_running {
                    let btn = ui.add_sized([120.0, 34.0],
                        egui::Button::new(egui::RichText::new(t(lang, "start")).size(16.0).strong().color(egui::Color32::BLACK))
                            .fill(egui::Color32::from_rgb(0, 180, 80)));
                    if btn.clicked() { self.start_server(); }
                } else {
                    let btn = ui.add_sized([120.0, 34.0],
                        egui::Button::new(egui::RichText::new(t(lang, "stop")).size(16.0).strong())
                            .fill(error));
                    if btn.clicked() { self.stop_server(); }
                }
                
                let btn = ui.add_sized([130.0, 34.0],
                    egui::Button::new(egui::RichText::new(t(lang, "open_webui")).size(14.0)));
                if btn.clicked() && self.is_running { self.open_web_ui(); }
                
                ui.separator();
                
                // 复制启动命令按钮
                if ui.button(egui::RichText::new("📋 Copy CMD").size(13.0)).clicked() {
                    self.copy_to_clipboard(&self.cmd_preview);
                    self.logs.push("Command copied to clipboard.".to_string());
                }
            });
        });

        // ========== 底部面板（压缩高度） ==========
        egui::TopBottomPanel::bottom("console_panel").min_height(100.0).max_height(180.0).show(ctx, |ui| {
            // 工具栏
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(t(lang, "console")).size(13.0).strong());
                ui.separator();
                if ui.button(t(lang, "clear")).clicked() {
                    self.logs.clear();
                    self.logs.push(t(lang, "console_cleared").to_string());
                }
                if ui.button("📋 Copy All").clicked() {
                    let all_logs = self.logs.join("\n");
                    self.copy_to_clipboard(&all_logs);
                    self.logs.push("All logs copied to clipboard.".to_string());
                }
                if ui.button("💾 Save Log").clicked() {
                    // 用简单时间戳作为文件名
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    if let Some(path) = rfd::FileDialog::new()
                        .set_file_name(&format!("llama_log_{}.txt", now))
                        .save_file()
                    {
                        let content = self.logs.join("\n");
                        if let Err(e) = std::fs::write(&path, content) {
                            self.logs.push(format!("Failed to save log: {}", e));
                        } else {
                            self.logs.push(format!("Log saved to: {}", path.display()));
                        }
                    }
                }
                ui.separator();
                ui.label(egui::RichText::new(format!("{} lines", self.logs.len())).size(11.0).monospace());
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 命令输入
                    ui.label(">");
                    let response = ui.add(egui::TextEdit::singleline(&mut self.console_input)
                        .desired_width(200.0).font(egui::TextStyle::Monospace));
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.execute_console_command();
                    }
                    if ui.button(t(lang, "run")).clicked() {
                        self.execute_console_command();
                    }
                });
            });
            
            // 可选择的日志文本区域
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(5, 5, 8))
                .rounding(4.0)
                .inner_margin(4.0)
                .show(ui, |ui| {
                    // 将日志拼接成一个可复制的字符串
                    let log_text = self.logs.join("\n");
                    let mut log_edit = log_text.clone();
                    ui.add(egui::TextEdit::multiline(&mut log_edit)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(4)
                        .desired_width(ui.available_width())
                        .interactive(true)); // 允许选择和复制
                });
        });

        // ========== 中央区域：三栏布局 ==========
        // 左：模型/网络/GPU  |  中：性能/选项  |  右：预设+运行时参数
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                // ===== 左栏：模型与服务器 + 网络 + GPU与内存 =====
                let left_width = (ui.available_width() * 0.35).floor();
                ui.vertical(|ui| {
                    ui.set_width(left_width);
                    ui.add_space(8.0);
                    
                    egui::ScrollArea::vertical()
                        .id_source("left_scroll")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            
                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "model_server")).size(16.0).color(accent))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "model"));
                                        let resp = ui.add(egui::TextEdit::singleline(&mut self.launch_config.model_path)
                                            .desired_width(ui.available_width() - 70.0));
                                        if resp.changed() {
                                            self.on_launch_param_changed();
                                        }
                                        if ui.button(t(lang, "browse")).clicked() { self.open_model_dialog(); }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "server"));
                                        let resp = ui.add(egui::TextEdit::singleline(&mut self.launch_config.llama_server_path)
                                            .desired_width(ui.available_width() - 70.0));
                                        if resp.changed() {
                                            self.on_launch_param_changed();
                                        }
                                        if ui.button(t(lang, "browse")).clicked() { self.open_exe_dialog(); }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "mmproj"));
                                        let resp = ui.add(egui::TextEdit::singleline(&mut self.launch_config.mmproj_path)
                                            .desired_width(ui.available_width() - 110.0));
                                        if resp.changed() {
                                            self.on_launch_param_changed();
                                        }
                                        if ui.button(t(lang, "browse")).clicked() { self.open_mmproj_dialog(); }
                                        if ui.button(t(lang, "clear")).clicked() {
                                            self.launch_config.mmproj_path.clear();
                                            self.update_command_preview();
                                            self.save_config();
                                        }
                                    });
                                });

                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "network")).size(16.0).color(accent))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "host"));
                                        let host_resp = ui.add(egui::TextEdit::singleline(&mut self.launch_config.host).desired_width(ui.available_width() - 140.0));
                                        ui.label(t(lang, "port"));
                                        let port_resp = ui.add(egui::DragValue::new(&mut self.launch_config.port).speed(1).range(1024..=65535));
                                        if host_resp.changed() || port_resp.changed() {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                });

                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "gpu_memory")).size(16.0).color(accent))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "gpu_layers"));
                                        let sw = ui.available_width() - 50.0;
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.launch_config.n_gpu_layers, 0..=999)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                        if ui.button(t(lang, "max")).clicked() { 
                                            self.launch_config.n_gpu_layers = 999; 
                                            self.on_launch_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "ctx_size"));
                                        let sw = ui.available_width() - 90.0;
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.launch_config.ctx_size, 512..=999999).logarithmic(true)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                        if ui.add(egui::DragValue::new(&mut self.launch_config.ctx_size).speed(512).range(512..=999999)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                    // parallel_slots > 1 时提示 ctx_size 会被均分
                                    if self.launch_config.n_parallel > 1 {
                                        let per_seq = self.launch_config.ctx_size / self.launch_config.n_parallel;
                                        ui.label(egui::RichText::new(format!("⚠ ctx/seq = {} (total ÷ parallel)", per_seq)).size(11.0).color(egui::Color32::YELLOW));
                                    }
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "parallel_slots"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.launch_config.n_parallel, 1..=16)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "kv_cache"));
                                        let kv_changed = egui::ComboBox::from_id_source("kv_cache").width(ui.available_width())
                                            .selected_text(&self.launch_config.kv_cache_type)
                                            .show_ui(ui, |ui| {
                                                for t in &["f16", "f32", "q8_0", "q4_0", "q4_1", "q5_0", "q5_1", "bf16", "iq4_nl"] {
                                                    ui.selectable_value(&mut self.launch_config.kv_cache_type, t.to_string(), *t);
                                                }
                                            }).inner.is_some();
                                        if kv_changed {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                });
                        });
                });

                ui.separator();

                // ===== 中栏：性能 + 选项 =====
                let mid_width = (ui.available_width() * 0.28).floor();
                ui.vertical(|ui| {
                    ui.set_width(mid_width);
                    ui.add_space(8.0);
                    
                    egui::ScrollArea::vertical()
                        .id_source("mid_scroll")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            
                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "performance")).size(16.0).color(accent))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "threads"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.launch_config.threads, 1..=32)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "batch_threads"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.launch_config.threads_batch, 1..=32)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "batch_size"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.launch_config.batch_size, 256..=8192).logarithmic(true)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "micro_batch"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.launch_config.ubatch_size, 64..=2048)).changed() {
                                            self.on_launch_param_changed();
                                        }
                                    });
                                });

                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "options")).size(16.0).color(accent))
                                .default_open(true)
                                .show(ui, |ui| {
                                    if ui.checkbox(&mut self.launch_config.flash_attn, t(lang, "flash_attn")).changed() {
                                        self.on_launch_param_changed();
                                    }
                                    if ui.checkbox(&mut self.launch_config.warmup, t(lang, "warmup")).changed() {
                                        self.on_launch_param_changed();
                                    }
                                    if ui.checkbox(&mut self.launch_config.continuous_batching, t(lang, "cont_batching")).changed() {
                                        self.on_launch_param_changed();
                                    }
                                    if ui.checkbox(&mut self.launch_config.verbose, t(lang, "verbose")).changed() {
                                        self.on_launch_param_changed();
                                    }
                                });
                        });
                });

                ui.separator();

                // ===== 右栏：预设 + 运行时参数 =====
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.add_space(8.0);
                    
                    egui::ScrollArea::vertical()
                        .id_source("right_scroll")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            
                            // 预设区域
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(t(lang, "preset")).size(16.0).color(accent).strong());
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button(t(lang, "save")).clicked() { self.show_save_preset_dialog = true; }
                                });
                            });
                            
                            // 预设选择栏不要太长
                            let preset_text = self.settings.last_preset_index
                                .and_then(|i| self.settings.presets.get(i))
                                .map(|p| p.name.clone())
                                .unwrap_or_else(|| t(lang, "select_preset"));
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_id_source("preset_selector")
                                    .width((ui.available_width() - 60.0).min(180.0))
                                    .selected_text(&preset_text)
                                    .show_ui(ui, |ui| {
                                        let preset_count = self.settings.presets.len();
                                        let mut clicked_idx: Option<usize> = None;
                                        for i in 0..preset_count {
                                            let name = self.settings.presets[i].name.clone();
                                            if ui.selectable_label(false, name).clicked() {
                                                clicked_idx = Some(i);
                                            }
                                        }
                                        if let Some(idx) = clicked_idx { self.apply_preset(idx); }
                                    });
                                
                                if ui.button(t(lang, "apply")).clicked() {
                                    if let Some(idx) = self.settings.last_preset_index { self.apply_preset(idx); }
                                }
                            });

                            // 预设管理按钮行（强制覆盖 / 还原默认 / 删除）
                            if self.settings.last_preset_index.is_some() {
                                ui.horizontal(|ui| {
                                    if ui.button(egui::RichText::new(t(lang, "force_overwrite")).size(12.0)).clicked() {
                                        self.force_overwrite_preset();
                                    }
                                    if ui.button(egui::RichText::new(t(lang, "restore_default")).size(12.0)).clicked() {
                                        self.restore_preset_default();
                                    }
                                    if ui.button(egui::RichText::new(t(lang, "delete_preset")).size(12.0)).clicked() {
                                        if let Some(idx) = self.settings.last_preset_index {
                                            self.confirm_delete_preset_idx = Some(idx);
                                        }
                                    }
                                });
                            }

                            // 删除确认提示
                            if let Some(del_idx) = self.confirm_delete_preset_idx {
                                let del_name = self.settings.presets.get(del_idx)
                                    .map(|p| p.name.clone())
                                    .unwrap_or_default();
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("{} {}?", t(lang, "confirm_delete"), del_name))
                                        .size(12.0).color(egui::Color32::YELLOW));
                                    if ui.button("✓").clicked() {
                                        self.delete_preset(del_idx);
                                        self.confirm_delete_preset_idx = None;
                                    }
                                    if ui.button("✗").clicked() {
                                        self.confirm_delete_preset_idx = None;
                                    }
                                });
                            }
                            
                            if !self.params_apply_status.is_empty() {
                                let color = if self.params_modified { egui::Color32::YELLOW } else { success };
                                ui.label(egui::RichText::new(&self.params_apply_status).size(12.0).color(color));
                            }

                            ui.add_space(4.0);

                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "temperature_sampling")).size(15.0).color(accent))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "temperature"));
                                        let sw = ui.available_width() - 80.0;
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.temperature, 0.0..=2.0)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                        if ui.add(egui::DragValue::new(&mut self.runtime_params.temperature).speed(0.01).range(0.0..=2.0)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "top_k"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.top_k, 0..=100)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "top_p"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.top_p, 0.0..=1.0)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "min_p"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.min_p, 0.0..=1.0)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                });

                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "repetition_control")).size(15.0).color(accent))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "repeat_penalty"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.repeat_penalty, 1.0..=2.0)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "check_range"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.repeat_last_n, 0..=512)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "presence_penalty"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.presence_penalty, -2.0..=2.0)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "frequency_penalty"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.frequency_penalty, -2.0..=2.0)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                });

                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "mirostat")).size(15.0).color(accent))
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Mode:");
                                        egui::ComboBox::from_id_source("mirostat").width(120.0)
                                            .selected_text(match self.runtime_params.mirostat {
                                                0 => t(lang, "disabled"), 1 => t(lang, "mirostat_v1"), 2 => t(lang, "mirostat_v2"), _ => "Unknown".to_string(),
                                            })
                                            .show_ui(ui, |ui| {
                                                if ui.selectable_value(&mut self.runtime_params.mirostat, 0, t(lang, "disabled")).clicked() {
                                                    self.on_runtime_param_changed();
                                                }
                                                if ui.selectable_value(&mut self.runtime_params.mirostat, 1, t(lang, "mirostat_v1")).clicked() {
                                                    self.on_runtime_param_changed();
                                                }
                                                if ui.selectable_value(&mut self.runtime_params.mirostat, 2, t(lang, "mirostat_v2")).clicked() {
                                                    self.on_runtime_param_changed();
                                                }
                                            });
                                    });
                                    if self.runtime_params.mirostat > 0 {
                                        ui.horizontal(|ui| {
                                            ui.label(t(lang, "tau"));
                                            let sw = ui.available_width();
                                            if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.mirostat_tau, 1.0..=10.0)).changed() {
                                                self.on_runtime_param_changed();
                                            }
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label(t(lang, "eta"));
                                            let sw = ui.available_width();
                                            if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.mirostat_eta, 0.01..=1.0)).changed() {
                                                self.on_runtime_param_changed();
                                            }
                                        });
                                    }
                                });

                            egui::CollapsingHeader::new(egui::RichText::new(t(lang, "other")).size(15.0).color(accent))
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "max_tokens"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.n_predict, -1..=8192)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(t(lang, "seed"));
                                        let sw = ui.available_width();
                                        if ui.add_sized([sw, 22.0], egui::Slider::new(&mut self.runtime_params.seed, -1..=999999)).changed() {
                                            self.on_runtime_param_changed();
                                        }
                                    });
                                });

                            if self.params_modified {
                                let btn = ui.add_sized([ui.available_width(), 32.0],
                                    egui::Button::new(egui::RichText::new(t(lang, "apply_params")).size(14.0).strong()).fill(accent));
                                if btn.clicked() { self.apply_runtime_params(); }
                            }
                        });
                });
            });
        });

        // 保存预设对话框
        if self.show_save_preset_dialog {
            let mut should_close = false;
            egui::Window::new(t(lang, "save_preset")).collapsible(false).resizable(false).show(ctx, |ui| {
                ui.label(t(lang, "preset_name"));
                ui.text_edit_singleline(&mut self.new_preset_name);
                ui.label(t(lang, "description"));
                ui.text_edit_singleline(&mut self.new_preset_description);
                ui.horizontal(|ui| {
                    if ui.button(t(lang, "save")).clicked() { self.save_current_as_preset(); }
                    if ui.button(t(lang, "cancel")).clicked() { should_close = true; }
                });
            });
            if should_close {
                self.show_save_preset_dialog = false;
            }
        }

        // 所有命令对话框
        if self.show_all_commands {
            let cmds = self.quick_commands.clone();
            let mut should_close = false;
            egui::Window::new(t(lang, "all_cmds")).collapsible(true).resizable(true).show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Built-in Commands:");
                    for cmd in &cmds {
                        let cmd_name = cmd.name.clone();
                        let cmd_desc = cmd.description.clone();
                        let cmd_cmd = cmd.command.clone();
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(&cmd_name).size(13.0).strong());
                            ui.label(&cmd_desc);
                            ui.monospace(&cmd_cmd);
                            ui.horizontal(|ui| {
                                if ui.button(t(lang, "copy")).clicked() { 
                                    self.copy_to_clipboard(&cmd_cmd);
                                }
                            });
                        });
                    }
                });
                if ui.button(t(lang, "close")).clicked() { 
                    should_close = true;
                }
            });
            if should_close || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.show_all_commands = false;
            }
        }

        if self.is_running {
            let mut process = self.server_process.lock().unwrap();
            if let Some(ref mut child) = *process {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        drop(process);
                        self.is_running = false;
                        let lang = self.settings.language;
                        if status.success() {
                            self.status_message = t(lang, "status_stopped").to_string();
                            self.logs.push(t(lang, "server_stopped").to_string());
                        } else {
                            self.status_message = format!("Process exited: {}", status);
                            self.logs.push(t(lang, "server_crashed").to_string());
                            self.logs.push(format!("Exit code: {:?}", status.code()));
                            self.logs.push("Check if llama-server path and model path are correct.".to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_config();
        self.stop_server();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 750.0])
            .with_min_inner_size([900.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native("DeveLlamaGUI Pro", options, Box::new(|cc| Ok(Box::new(DeveLlamaGUI::new(cc)))))
}
