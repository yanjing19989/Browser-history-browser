// 设置页面脚本
const { invoke } = window.__TAURI__.tauri;
const { ask } = window.__TAURI__.dialog;

const elements = {
  backBtn: document.getElementById('backBtn'),
  dbPath: document.getElementById('dbPath'),
  browseBtn: document.getElementById('browseBtn'),
  dbStatus: document.getElementById('dbStatus'),
  validateBtn: document.getElementById('validateBtn'),
  applyBtn: document.getElementById('applyBtn'),
  openDirBtn: document.getElementById('openDirBtn'),
  cleanupBtn: document.getElementById('cleanupBtn'),
  toast: document.getElementById('messageToast'),
  toastMessage: document.getElementById('toastMessage'),
  // 新增的浏览器同步相关元素
  browserDbPath: document.getElementById('browserDbPath'),
  browseBrowserBtn: document.getElementById('browseBrowserBtn'),
  syncBtn: document.getElementById('syncBtn'),
  syncStatus: document.getElementById('syncStatus')
};

let currentConfig = null;

function showToast(message, type = 'info') {
  // 移除已存在的提示
  const existingToast = document.querySelector('.toast');
  if (existingToast) {
    existingToast.remove();
  }

  // 创建新的提示
  const toast = document.createElement('div');
  toast.className = `toast toast-${type}`;
  toast.textContent = message;
  document.body.appendChild(toast);

  // 显示提示
  setTimeout(() => toast.classList.add('show'), 100);

  // 3秒后自动隐藏
  setTimeout(() => {
    toast.classList.remove('show');
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}

// 更新状态显示
function updateStatus(status, message) {
  const statusText = elements.dbStatus.querySelector('.status-text');
  statusText.textContent = message;
  statusText.className = `status-text status-${status}`;
}

// 更新按钮状态
function updateButtons() {
  const hasPath = elements.dbPath.value.trim() !== '';
  const hasValidConfig = currentConfig && currentConfig.db_path;

  elements.validateBtn.disabled = !hasPath;
  elements.applyBtn.disabled = !hasPath;
  elements.openDirBtn.disabled = !hasValidConfig;
  elements.cleanupBtn.disabled = !hasValidConfig;
}

// 加载当前配置
async function loadConfig() {
  try {
    currentConfig = await invoke('get_config');
    if (currentConfig.db_path) {
      elements.dbPath.value = currentConfig.db_path;
      updateStatus('ok', '已配置数据库路径');
    } else {
      updateStatus('warning', '未配置数据库路径');
    }

    // 加载浏览器数据库路径
    if (currentConfig.browser_db_path) {
      elements.browserDbPath.value = currentConfig.browser_db_path;
      updateSyncStatus('ok', '已保存浏览器数据库路径');
    }

    updateButtons();
    updateSyncButtons();
  } catch (error) {
    console.error('加载配置失败:', error);
    updateStatus('error', '配置加载失败');
  }
}

// 浏览文件
async function browseFile() {
  try {
    const filePath = await invoke('browse_db_file');
    if (filePath) {
      elements.dbPath.value = filePath;
      updateButtons();
      updateStatus('warning', '路径已选择，请验证并应用');
    }
  } catch (error) {
    console.error('文件选择失败:', error);
    showToast('文件选择失败: ' + error, 'error');
  }
}

// 验证数据库路径
async function validatePath() {
  const path = elements.dbPath.value.trim();
  if (!path) return;

  try {
    elements.validateBtn.disabled = true;
    elements.validateBtn.textContent = '验证中...';

    const isValid = await invoke('validate_db_path', { path });

    if (isValid) {
      updateStatus('ok', '数据库路径有效');
      showToast('数据库路径验证成功', 'success');
    } else {
      updateStatus('error', '数据库路径无效');
      showToast('数据库路径无效', 'error');
    }
  } catch (error) {
    console.error('验证失败:', error);
    updateStatus('error', '验证失败: ' + error);
    showToast('验证失败: ' + error, 'error');
  } finally {
    elements.validateBtn.disabled = false;
    elements.validateBtn.textContent = '验证路径';
    updateButtons();
  }
}

// 应用设置
async function applySettings() {
  const path = elements.dbPath.value.trim();
  if (!path) return;

  try {
    elements.applyBtn.disabled = true;
    elements.applyBtn.textContent = '应用中...';

    const result = await invoke('set_db_path', { path });

    updateStatus('ok', '数据库配置成功');
    showToast(result, 'success');

    // 更新当前配置
    currentConfig = await invoke('get_config');

  } catch (error) {
    console.error('应用设置失败:', error);
    updateStatus('error', '设置失败: ' + error);
    showToast('设置失败: ' + error, 'error');
  } finally {
    elements.applyBtn.disabled = false;
    elements.applyBtn.textContent = '应用设置';
    updateButtons();
  }
}

// 返回主页
function goBack() {
  window.location.href = 'index.html';
}

// 事件监听
elements.backBtn.addEventListener('click', goBack);
elements.browseBtn.addEventListener('click', browseFile);
elements.validateBtn.addEventListener('click', validatePath);
elements.applyBtn.addEventListener('click', applySettings);
elements.openDirBtn.addEventListener('click', openDbDirectory);
elements.cleanupBtn.addEventListener('click', cleanupOldDbs);
elements.dbPath.addEventListener('input', updateButtons);

// 新增的浏览器同步功能事件监听
elements.browseBrowserBtn.addEventListener('click', browseBrowserFile);
elements.syncBtn.addEventListener('click', syncBrowserDb);
elements.browserDbPath.addEventListener('input', updateSyncButtons);

// 键盘快捷键
window.addEventListener('keydown', e => {
  if (e.key === 'Escape') goBack();
});

// 复制路径按钮事件监听
document.addEventListener('click', (e) => {
  if (e.target.classList.contains('copy-path-btn')) {
    copyPathToClipboard(e.target.dataset.path);
  }
});

// 复制路径到剪贴板
async function copyPathToClipboard(path) {
  try {
    // 对于Windows环境变量，提供基础路径供用户参考
    let displayPath = path;
    if (path.includes('%LOCALAPPDATA%')) {
      displayPath = path; // 保持原始格式，用户可以手动替换
    } else if (path.includes('%APPDATA%')) {
      displayPath = path; // 保持原始格式，用户可以手动替换
    }

    await navigator.clipboard.writeText(displayPath);
    showToast('路径已复制到剪贴板，请根据需要替换环境变量');
  } catch (error) {
    console.error('复制失败:', error);
    // 降级方案：使用传统方法
    try {
      const textArea = document.createElement('textarea');
      textArea.value = path;
      document.body.appendChild(textArea);
      textArea.select();
      document.execCommand('copy');
      document.body.removeChild(textArea);
      showToast('路径已复制到剪贴板');
    } catch (fallbackError) {
      showToast('复制失败，请手动复制', 'error');
    }
  }
}

// 浏览浏览器数据库文件
async function browseBrowserFile() {
  try {
    const filePath = await invoke('browse_browser_db_file');
    if (filePath) {
      elements.browserDbPath.value = filePath;
      updateSyncButtons();
      updateSyncStatus('warning', '已选择文件，可以同步');

      // 保存浏览器数据库路径到配置
      try {
        await invoke('set_browser_db_path', { path: filePath });
        showToast('浏览器数据库路径已保存', 'success');
      } catch (saveError) {
        console.warn('保存浏览器数据库路径失败:', saveError);
      }
    }
  } catch (error) {
    console.error('浏览器文件选择失败:', error);
    showToast('文件选择失败: ' + error, 'error');
  }
}

// 更新同步状态显示
function updateSyncStatus(status, message) {
  const statusText = elements.syncStatus.querySelector('.status-text');
  statusText.textContent = message;
  statusText.className = `status-text status-${status}`;
}

// 更新同步按钮状态
function updateSyncButtons() {
  const hasBrowserPath = elements.browserDbPath.value.trim() !== '';
  elements.syncBtn.disabled = !hasBrowserPath;
}

// 同步浏览器数据库
async function syncBrowserDb() {
  const browserPath = elements.browserDbPath.value.trim();
  if (!browserPath) return;

  try {
    elements.syncBtn.disabled = true;
    elements.syncBtn.textContent = '📥 同步中...';
    updateSyncStatus('warning', '正在同步数据库...');

    // 复制浏览器数据库到程序目录
    const appDbPath = await invoke('copy_browser_db_to_app', { sourcePath: browserPath });

    // 自动设置为当前数据库路径
    elements.dbPath.value = appDbPath;
    await applySettings();

    updateSyncStatus('ok', '同步成功');
    showToast('浏览器数据库同步成功!', 'success');

    // 清空浏览器路径
    elements.browserDbPath.value = '';
    updateSyncButtons();

  } catch (error) {
    console.error('同步失败:', error);
    updateSyncStatus('error', '同步失败: ' + error);
    showToast('同步失败: ' + error, 'error');
  } finally {
    elements.syncBtn.disabled = false;
    elements.syncBtn.textContent = '📥 同步到程序';
    updateSyncButtons();
  }
}

// 打开数据库所在目录
async function openDbDirectory() {
  try {
    elements.openDirBtn.disabled = true;
    const result = await invoke('open_db_directory');
    showToast(result, 'success');
  } catch (error) {
    console.error('打开目录失败:', error);
    showToast('打开目录失败: ' + error, 'error');
  } finally {
    elements.openDirBtn.disabled = false;
    updateButtons();
  }
}

// 自动清理旧数据库文件
async function cleanupOldDbs() {
  const confirmed = await ask('确定要清理除当前使用外的所有.db文件吗？此操作不可撤销！', {
    title: '确认清理',
    type: 'warning',
    okLabel: '确定清理',
    cancelLabel: '取消'
  });

  if (!confirmed) {
    return;
  }

  try {
    elements.cleanupBtn.disabled = true;
    const result = await invoke('cleanup_old_dbs');
    showToast(result, 'success');
  } catch (error) {
    console.error('清理失败:', error);
    showToast('清理失败: ' + error, 'error');
  } finally {
    elements.cleanupBtn.disabled = false;
    updateButtons();
  }
}

// 初始化
loadConfig();
updateSyncButtons();
