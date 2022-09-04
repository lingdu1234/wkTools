<template>
  <el-config-provider :locale="localeLang">
    <div v-if="is_started">
      <div @contextmenu="handleMouse" :style="'opacity:' + appStore.opacity / 1000 + ';'">
        <el-header>
          <el-container @mousedown="dragWindow">
            <!-- 头部标题 -->
            <div class="title_logo" @click="gotoAdminHome">
              <img src="@/assets/logo2.png" />
              <span>{{ t(appStore.routeInfo.title) }}</span>
            </div>
          </el-container>
          <div class="operatorClass">
            <div class="operatorClassA">
              <!-- 最上层切换 -->
              <div>
                <el-icon v-if="isDark" @click="set_always_on_top">
                  <SvgIcon :name="is_on_top ? 'pined' : 'pinDark'"></SvgIcon>
                </el-icon>
                <el-icon v-else @click="set_always_on_top">
                  <SvgIcon :name="is_on_top ? 'pined' : 'pin'"></SvgIcon>
                </el-icon>
              </div>
              <!-- 透明度切换 -->
              <el-popover>
                <template #reference>
                  <el-icon>
                    <SvgIcon :name="isDark ? 'setOpacityDark' : 'setOpacity'"></SvgIcon>
                  </el-icon>
                </template>
                <el-slider v-model="appStore.opacity" :format-tooltip="formatTooltip" :min="600" :max="1000" />
              </el-popover>
              <!-- 语言切换 -->
              <div>
                <el-icon v-if="locale === 'zh'" @click="changeLang('en')">
                  <SvgIcon :name="isDark ? 'enDark' : 'en'"></SvgIcon>
                </el-icon>
                <el-icon v-else @click="changeLang('zh')">
                  <SvgIcon :name="isDark ? 'zhDark' : 'zh'"></SvgIcon>
                </el-icon>
              </div>
              <!-- 黑暗模式切换 -->
              <div>
                <el-icon v-if="isDark" @click="toggleDark()">
                  <Moon />
                </el-icon>
                <el-icon v-else @click="toggleDark()">
                  <Sunny />
                </el-icon>
              </div>
            </div>
            <div class="operatorClassB">
              <el-icon @click="minimize">
                <Minus />
              </el-icon>
              <el-icon @click="maximize">
                <Plus />
              </el-icon>
              <el-icon @click="titlebarClose">
                <CloseBold />
              </el-icon>
            </div>
          </div>
        </el-header>
        <el-container>
          <!-- 侧边栏 -->
          <!-- @changeTitle="change_title" -->
          <SideBar></SideBar>
          <el-container>
            <el-main>
              <el-scrollbar :height="window_h - 30">
                <!-- 路由占位符 -->
                <div class="main-content">
                  <router-view />
                </div>
              </el-scrollbar>
            </el-main>
          </el-container>
        </el-container>
      </div>
    </div>
  </el-config-provider>
  <Updater :is_show="is_show_dialog" @closeUpdateDialog="closeUpdateDialog" />
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { checkUpdate } from '@tauri-apps/api/updater'
import { resourceDir, appDir, resolveResource } from '@tauri-apps/api/path';
import SvgIcon from '@/components/SvgIcon.vue';
import SideBar from "@/components/SideBar"
import { Minus, Plus, CloseBold, Sunny, Moon } from '@element-plus/icons-vue';
import { useDark, useToggle } from '@vueuse/core';
import useAppStore from "@/store/modules/app"

import { useI18n } from 'vue-i18n';
import i18n from '@/locals'; // 当前语言

import zh from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'

const appStore = useAppStore();

const { locale } = useI18n();

const { t } = i18n.global;

// 当前是否为暗色模式
const isDark = useDark();

// 是否最上层
const is_on_top = ref(false);

const toggleDark = useToggle(isDark);


//  软件是否已经启动
const is_started = ref(false)

//  显示更新对话框
const is_show_dialog = ref(false);

//  更新信息
const up_info = ref({
  version: "",
  date: "",
  body: ""
})


const formatTooltip = (val) => {
  return Number(val / 1000).toFixed(3)
}


// 是否最大化
const isMaximize = ref(false);


const window_h = ref(null)

onMounted(() => {
  window.onresize = () => {
    getWindowResize()
  };
})

const reload = () => {
  is_started.value = false
  nextTick(() => {
    is_started.value = true
  })
}

const getWindowResize = () => {
  nextTick(() => {
    window_h.value = document.documentElement.clientHeight;
  })
}

const localeLang = computed(() => (appStore.lang === 'zh' ? zh : en))


//  窗口操作
// 最小化
async function minimize() {
  await appWindow.minimize();
}
// 最大化
async function maximize() {
  if (isMaximize.value) {
    await appWindow.unmaximize();
    isMaximize.value = false;
    getWindowResize()

  } else {
    await appWindow.maximize();
    isMaximize.value = true;
    getWindowResize()
  }
}
// 关闭
function titlebarClose() {
  appWindow.close();
}
// 拖动窗口
async function dragWindow(e) {
  await appWindow.startDragging();
}

// 切换语言
async function changeLang(v) {
  locale.value = v;
  await appStore.setLang(v);
  reload()
}

async function set_always_on_top() {
  is_on_top.value = !is_on_top.value
  await appWindow.setAlwaysOnTop(is_on_top.value)
}

// 设置路径
async function set_path() {
  const res_path = await resourceDir();
  const blank_db_path = await resolveResource('__data/database/db_blank.db');
  const db_sql = await resolveResource('__data/sql');
  const appDirPath = await appDir();
  let v = [
    {
      key: "RES_PATH",
      path: res_path
    },
    {
      key: "BLANK_DB_PATH",
      path: blank_db_path
    },
    {
      key: "DB_SQL",
      path: db_sql
    },
    {
      key: "APP_PATH",
      path: appDirPath
    },
  ]
  await invoke("set_path_js", { v })
}

const check_update = async () => {
  await appStore.setUpdatable(false)
  try {
    const { shouldUpdate, manifest } = await checkUpdate()
    if (shouldUpdate) {
      const { body, date, version } = manifest;
      up_info.value = {
        version: version,
        date: date,
        body: body
      }
      await appStore.setUpdatable(true)
      await appStore.setUpdateInfo(up_info.value)
      is_show_dialog.value = true
    }
  } catch (e) {

  }

}

// 关闭更新对话框回调函数
const closeUpdateDialog = () => {
  is_show_dialog.value = false
}

// 添加监听函数，监听 DOM 内容加载完成事件
document.addEventListener('DOMContentLoaded', async () => {
  await set_path();
  await invoke("init_database");
  await changeLang(appStore.lang || "zh")
  await appStore.appInit()

  is_started.value = true

  // DOM 内容加载完成之后，通过 invoke 调用 在 Rust 中已经注册的命令
  await invoke('close_splashscreen')


  setTimeout(async () => {
    await check_update();
  }, 1500)
})

const handleMouse = (e) => {
  if (process.env.NODE_ENV === "production") {
    e.preventDefault();
  }

}
getWindowResize();
</script>

<style lang="scss" scoped>
.el-header {
  border-radius: 15px 15px 0px 0px;
  height: 30px;
  background-color: #bbe6d6;
  color: black;
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;

  .operatorClass {
    width: 200px;
    display: flex;
    justify-content: space-between;

    .operatorClassA {
      width: 100px;
      display: flex;
      justify-content: space-around;
    }

    .operatorClassB {
      width: 80px;
      display: flex;
      justify-content: space-between;
      size: 36px;

      i {
        cursor: pointer;
      }
    }
  }

  .title_logo {
    display: flex;
    justify-content: start;

    img {
      height: 22px;
      width: 22px;
    }

    span {
      margin-left: 10px;
      font-size: 18px;
    }
  }
}


.el-main {
  background-color: #d9f1e8;
  color: rgb(14, 13, 13);
  height: 100vh;
  padding: 0;
  .main-content{
    padding: 20px;
  }
}

html.dark {
  .el-header {
    background-color: rgb(13, 17, 23);
    color: #d9f1e8;
  }

  .el-main {
    background-color: rgb(1, 4, 9);
    color: rgb(216, 203, 203);
  }
}
</style>
