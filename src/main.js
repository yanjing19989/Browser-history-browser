// 基础前端脚本：调用 Tauri 后端命令并渲染
// 使用 Tauri JS API (window.__TAURI__) 进行 invoke

const { invoke } = window.__TAURI__.tauri;

const state = {
  page: 1,
  pageSize: 20,
  total: 0,
  keyword: '',
  timeRange: '7d',
  startDate: '',
  endDate: '',
  locale: '',
  items: []
};

async function fetchStats(){
  try {
    // 处理统计的时间范围
    let timeRange = state.timeRange;
    if (state.timeRange === 'custom' && state.startDate && state.endDate) {
      const startTs = Math.floor(new Date(state.startDate + 'T00:00:00').getTime() / 1000);
      const endTs = Math.floor(new Date(state.endDate + 'T23:59:59').getTime() / 1000);
      timeRange = `${startTs}-${endTs}`;
    }
    
    const stats = await invoke('stats_overview', { time_range: timeRange });
    renderKpis(stats);
  } catch(e){ 
    console.error('获取统计失败:', e); 
  }
}

async function fetchList(){
  try {
    // 构建过滤器对象
    const filters = {
      keyword: state.keyword || null,
      locale: state.locale || null
    };
    
    // 处理时间范围
    if (state.timeRange === 'custom' && state.startDate && state.endDate) {
      // 自定义日期范围，转换为时间戳范围
      const startTs = Math.floor(new Date(state.startDate + 'T00:00:00').getTime() / 1000);
      const endTs = Math.floor(new Date(state.endDate + 'T23:59:59').getTime() / 1000);
      filters.time_range = `${startTs}-${endTs}`;
    } else if (state.timeRange !== 'all') {
      filters.time_range = state.timeRange;
    }
    
    console.log('发送过滤器:', filters); // 调试日志
    
    const res = await invoke('list_history', {
      page: state.page,
      pageSize: state.pageSize,
      filters: filters
    });
    state.items = res.items;
    state.total = res.total;
    renderTable();
  } catch(e){ 
    console.error('获取历史记录失败:', e); 
  }
}

function renderKpis(stats){
  const kpis = document.getElementById('kpis');
  kpis.innerHTML = '';
  const data = [
    { label: '总访问次数', value: stats.total_visits || 0 },
    { label: '唯一站点', value: stats.distinct_sites || 0 },
    { label: 'Top实体数', value: (stats.top_entities || []).length }
  ];
  data.forEach(k => {
    const div = document.createElement('div');
    div.className = 'kpi-card';
    div.innerHTML = `<h3>${k.label}</h3><div class="value">${k.value}</div>`;
    kpis.appendChild(div);
  });
}

function renderTable(){
  const tbody = document.getElementById('historyTBody');
  tbody.innerHTML = '';
  state.items.forEach(item => {
    const tr = document.createElement('tr');
    tr.innerHTML = `<td>${escapeHtml(item.title || '')}</td><td>${escapeHtml(shorten(item.url, 48))}</td><td>${fmtTime(item.last_visited_time)}</td><td>${item.num_visits}</td>`;
    tr.addEventListener('click', () => showDetail(item));
    tbody.appendChild(tr);
  });
  document.getElementById('pageInfo').textContent = `${state.page} / ${Math.max(1, Math.ceil(state.total / state.pageSize))}`;
}

function showDetail(item){
  const panel = document.getElementById('detailContent');
  panel.textContent = JSON.stringify(item, null, 2);
}

// Helpers
function escapeHtml(str){
  return str.replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;','\'':'&#39;'}[c]));
}
function shorten(str, len){ return str.length>len ? str.slice(0,len-3)+'...' : str; }
function fmtTime(ts){ 
  if(!ts || ts === 0) return '-'; 
  // ts 是从1970年1月1日UTC开始的秒数，需要转换为毫秒
  const d = new Date(ts * 1000); 
  // 使用本地时间格式显示
  return d.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit', 
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}

// Events
window.addEventListener('keydown', e=>{ if(e.ctrlKey && e.key.toLowerCase()==='k'){ e.preventDefault(); document.getElementById('searchInput').focus(); }});

document.getElementById('searchBtn').addEventListener('click', ()=>{
  state.keyword = document.getElementById('searchInput').value.trim();
  state.page = 1; fetchList();
});

document.getElementById('applyFilters').addEventListener('click', ()=>{
  state.timeRange = document.getElementById('timeRange').value;
  state.startDate = document.getElementById('startDate').value;
  state.endDate = document.getElementById('endDate').value;
  state.locale = document.getElementById('localeFilter').value.trim();
  state.page = 1; 
  fetchStats(); 
  fetchList();
});

// 时间范围切换事件
document.getElementById('timeRange').addEventListener('change', (e) => {
  const customDateRange = document.getElementById('customDateRange');
  if (e.target.value === 'custom') {
    customDateRange.style.display = 'block';
    // 设置默认日期范围（最近7天）
    const endDate = new Date();
    const startDate = new Date();
    startDate.setDate(startDate.getDate() - 7);
    
    document.getElementById('endDate').value = endDate.toISOString().split('T')[0];
    document.getElementById('startDate').value = startDate.toISOString().split('T')[0];
  } else {
    customDateRange.style.display = 'none';
  }
});

document.getElementById('prevPage').addEventListener('click', ()=>{ if(state.page>1){ state.page--; fetchList(); }});

document.getElementById('nextPage').addEventListener('click', ()=>{ if(state.page < Math.ceil(state.total/state.pageSize)){ state.page++; fetchList(); }});

document.getElementById('settingsBtn').addEventListener('click', ()=>{ window.location.href = 'settings.html'; });

// 初始加载
fetchStats();
fetchList();
