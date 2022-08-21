import { createI18n } from 'vue-i18n'
import zhLang from './lang/zh'
import enLang from './lang/en'

const messages = {
  en: { ...enLang },
  zh: { ...zhLang },
}
const language = (navigator.language || 'en').toLocaleLowerCase() // 这是获取浏览器的语言
const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: localStorage.getItem('lang') || language.split('-')[0] || 'zh', // 首先从缓存里拿，没有的话就用浏览器语言，
  fallbackLocale: 'zh', // 设置备用语言
  messages,
})

export default i18n