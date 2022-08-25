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
import { useDict } from '@/utils/dict'
import { resetForm, parseTime } from '@/utils/utl'

const app = createApp(App)

app.config.globalProperties.useDict = useDict
app.config.globalProperties.resetForm = resetForm
app.config.globalProperties.parseTime = parseTime


app.component('DictTag', DictTag)
app.component('Pagination', Pagination)
app.component('parseTime', parseTime)

app.use(router)
app.use(ElementPlus)

app.use(i18n);



app.mount('#app')