// 防止深色模式闪烁的初始化脚本
// 这个脚本必须在页面渲染前同步执行
(function () {
    // 获取存储的主题
    function getStoredTheme() {
        try {
            return localStorage.getItem('theme');
        } catch (e) {
            return null;
        }
    }

    // 检测系统主题偏好
    function getSystemTheme() {
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            return 'dark';
        }
        return 'light';
    }

    // 应用主题
    function applyInitialTheme() {
        const storedTheme = getStoredTheme() || 'auto';
        let effectiveTheme = storedTheme;

        if (storedTheme === 'auto') {
            effectiveTheme = getSystemTheme();
        }

        // 立即设置data-theme属性为实际主题，避免闪烁
        document.documentElement.setAttribute('data-theme', effectiveTheme);

        // 如果是深色模式，添加一个临时类来确保立即生效
        if (effectiveTheme === 'dark') {
            document.documentElement.classList.add('theme-dark-loading');
        }
    }

    // 立即执行
    applyInitialTheme();
})();
