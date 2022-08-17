import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import i18n from './locals'
import App from './App.vue'
import router from './router'
import 'virtual:svg-icons-register'

import './style.scss'
import './styleDark.scss'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'

import Pagination from '@/components/Pagination'
import DictTag from '@/components/DictTag'
import RightToolbar from '@/components/RightToolbar'
import { useDict } from '@/utils/dict'
import { resetForm } from '@/utils/utl'

const app = createApp(App)

app.config.globalProperties.useDict = useDict
app.config.globalProperties.resetForm = resetForm


app.component('DictTag', DictTag)
app.component('Pagination', Pagination)
app.component('RightToolbar', RightToolbar)

app.use(router)
app.use(ElementPlus)

app.use(i18n);


app.mount('#app')
