import { createI18n } from 'vue-i18n'
import { messages } from '../locales'

// 从 localStorage 获取保存的语言，默认为中文
const getDefaultLocale = (): string => {
  const savedLocale = localStorage.getItem('locale')
  if (savedLocale && (savedLocale === 'zh-CN' || savedLocale === 'en-US')) {
    return savedLocale
  }
  // 检测浏览器语言
  const browserLang = navigator.language
  if (browserLang.startsWith('zh')) {
    return 'zh-CN'
  }
  return 'en-US'
}

export const i18n = createI18n({
  legacy: false, // 使用 Composition API 模式
  locale: getDefaultLocale(),
  fallbackLocale: 'zh-CN',
  messages,
})

// 切换语言的辅助函数
export const setLocale = (locale: string) => {
  i18n.global.locale.value = locale as any
  localStorage.setItem('locale', locale)
  document.documentElement.lang = locale
}

