// 主题管理模块
class ThemeManager {
    constructor(){this.themes=['auto','light','dark'];this.currentTheme=this.getStoredTheme()||'auto';this.systemThemeQuery=window.matchMedia('(prefers-color-scheme: dark)');this.init()}
    init(){document.documentElement.classList.remove('theme-dark-loading');this.applyTheme(this.currentTheme);this.systemThemeQuery.addEventListener('change',()=>{if(this.currentTheme==='auto'){this.applyTheme('auto')}});this.initThemeSelector()}
    getStoredTheme(){try{return localStorage.getItem('theme')}catch(e){console.warn('无法访问localStorage:',e);return null}}
    setStoredTheme(theme){try{localStorage.setItem('theme',theme)}catch(e){console.warn('无法保存主题设置:',e)}}
    applyTheme(theme){let actualTheme=theme;if(theme==='auto'){actualTheme=this.systemThemeQuery.matches?'dark':'light'}document.documentElement.setAttribute('data-theme',actualTheme);this.currentTheme=theme;this.setStoredTheme(theme);this.updateThemeButton()}
    updateThemeButton(){const themeBtn=document.getElementById('themeBtn');if(!themeBtn)return;const themeConfig={'auto':{text:'🌓 自动',title:'自动模式 - 跟随系统'},'light':{text:'☀️ 浅色',title:'浅色模式'},'dark':{text:'🌙 深色',title:'深色模式'}};const config=themeConfig[this.currentTheme];if(config){themeBtn.textContent=config.text;themeBtn.title=config.title}}
    cycleTheme(){const currentIndex=this.themes.indexOf(this.currentTheme);const nextIndex=(currentIndex+1)%this.themes.length;const nextTheme=this.themes[nextIndex];this.applyTheme(nextTheme)}
    initThemeSelector(){const themeBtn=document.getElementById('themeBtn');if(!themeBtn)return;this.updateThemeButton();themeBtn.addEventListener('click',()=>{this.cycleTheme()})}
    getEffectiveTheme(){if(this.currentTheme==='auto'){return this.systemThemeQuery.matches?'dark':'light'}return this.currentTheme}
    isDarkMode(){return this.getEffectiveTheme()==='dark'}
}
// 全局主题管理器实例
window.themeManager = new ThemeManager();
