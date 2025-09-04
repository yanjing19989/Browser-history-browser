// 防止深色模式闪烁的初始化脚本，这个脚本必须在页面渲染前同步执行
(function () {
    function getStoredTheme(){try{return localStorage.getItem('theme')}catch(e){return null}}
    function getSystemTheme(){if(window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches){return'dark'}return'light'}
    function applyInitialTheme(){const storedTheme=getStoredTheme()||'auto';let effectiveTheme=storedTheme;if(storedTheme==='auto'){effectiveTheme=getSystemTheme()}document.documentElement.setAttribute('data-theme',effectiveTheme);if(effectiveTheme==='dark'){document.documentElement.classList.add('theme-dark-loading')}}
    applyInitialTheme();
})();
