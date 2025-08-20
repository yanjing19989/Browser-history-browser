// 设置页面脚本
const { invoke } = window.__TAURI__.tauri;

const elements = {
  backBtn: document.getElementById('backBtn'),
  dbPath: document.getElementById('dbPath'),
  browseBtn: document.getElementById('browseBtn'),
  dbStatus: document.getElementById('dbStatus'),
  validateBtn: document.getElementById('validateBtn'),
  applyBtn: document.getElementById('applyBtn'),
  toast: document.getElementById('messageToast'),
  toastMessage: document.getElementById('toastMessage')
};

let currentConfig = null;

// 显示提示消息
function showToast(message, type = 'info') {
  elements.toastMessage.textContent = message;
  elements.toast.className = `toast show ${type}`;
  setTimeout(() => {
    elements.toast.classList.remove('show');
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
  elements.validateBtn.disabled = !hasPath;
  elements.applyBtn.disabled = !hasPath;
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
    updateButtons();
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
elements.dbPath.addEventListener('input', updateButtons);

// 键盘快捷键
window.addEventListener('keydown', e => {
  if (e.key === 'Escape') goBack();
});

// 初始化
loadConfig();
