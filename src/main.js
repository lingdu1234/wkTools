import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import i18n from './locals'
import App from './App.vue'
import router from './router'
import 'virtual:svg-icons-register'

import zhCn from 'element-plus/es/locale/lang/zh-cn'

import './style.scss'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'

const app = createApp(App)

app.use(router)
app.use(ElementPlus, {
  locale: zhCn,
})

app.use(i18n);


app.mount('#app')
