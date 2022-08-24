<template>
  <el-config-provider :locale="localeLang">
    <div @contextmenu="handleMouse" :style="'opacity:' + opacity / 1000 + ';'">
      <el-header>
        <el-container @mousedown="dragWindow">
          <!-- 头部标题 -->
          <div class="title_logo" @click="gotoAdminHome">
            <img src="@/assets/logo2.png" />
            <span>{{ Headertitile }}</span>
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
              <el-slider v-model="opacity" :format-tooltip="formatTooltip" :min="600" :max="1000" />
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
        <el-aside>
          <div class="aside_logo" @click="gotoAdminHome">
            <img src="@/assets/logo2.png" />
            <span>WkTools</span>
          </div>
          <el-menu active-text-color="#5352ed" :default-active="activePath" unique-opened @select="handleMenuChange">
            <el-menu-item v-for="menu in menus" :index="menu.path" :key="menu.path"
              @click="getMenu(menu.path, menu.meta.title)">
              <el-icon>
                <SvgIcon :name="menu.meta.icon"></SvgIcon>
              </el-icon>
              <span>{{ menu.meta.title }}</span>
            </el-menu-item>
          </el-menu>
        </el-aside>
        <el-container>
          <el-main>
            <el-scrollbar :height="window_h">
              <!-- 路由占位符 -->
              <router-view />
            </el-scrollbar>
          </el-main>
        </el-container>
      </el-container>
    </div>
  </el-config-provider>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { resourceDir } from '@tauri-apps/api/path';
import { resolveResource } from '@tauri-apps/api/path';
import SvgIcon from '@/components/SvgIcon.vue';
import { constantRoutes as menus } from './router';
import { Minus, Plus, CloseBold, Sunny, Moon } from '@element-plus/icons-vue';
import { useDark, useToggle } from '@vueuse/core';

import { useI18n } from 'vue-i18n';
import i18n from '@/locals'; // 当前语言
const { locale } = useI18n();

import zh from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'

const { t } = i18n.global;

// 当前是否为暗色模式
const isDark = useDark();

// 是否最上层
const is_on_top = ref(false);

const toggleDark = useToggle(isDark);

const { proxy } = getCurrentInstance();

// 透明度
const opacity = ref(1000)

const formatTooltip = (val) => {
  return val / 1000
}



// 标题
const Headertitile = ref(t('app.welcome'));

// 是否最大化
const isMaximize = ref(false);

// 激活菜单
const activePath = ref('/index');

const window_h = ref(null)

onMounted(() => {
  activePath.value = localStorage.getItem("sfsafasfsa") || "/index"
  Headertitile.value = localStorage.getItem("wfw3t3t32t") || Headertitile.value
  proxy.$router.push(activePath.value);
  const v = localStorage.getItem('lang') || "zh";
  invoke('set_lang', { lang: v });
  window.onresize = () => {
    get_window_height()
  };
})


const get_window_height = () => {
  window_h.value = document.documentElement.clientHeight * 0.9;
}


const localeLang = computed(() => (locale.value === 'zh' ? zh : en))

function gotoAdminHome() {
  proxy.$router.push('/index');
  activePath.value = '/index';
}
function handleMenuChange(index, _indexPath) {
  activePath.value = index;
  proxy.$router.push(index);
}
function getMenu(index, title) {
  localStorage.setItem('sfsafasfsa', index)
  localStorage.setItem('wfw3t3t32t', title)
  Headertitile.value = title;
}
//  窗口操作
// 最小化
function minimize() {
  appWindow.minimize();
}
// 最大化
function maximize() {
  if (isMaximize.value) {
    appWindow.unmaximize();
    isMaximize.value = false;
  } else {
    appWindow.maximize();
    isMaximize.value = true;
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
  await invoke('set_lang', { lang: v });
  localStorage.setItem('lang', v);
  window.location.reload();
}
async function set_always_on_top() {
  is_on_top.value = !is_on_top.value
  await appWindow.setAlwaysOnTop(is_on_top.value)
}

// 设置路径
async function set_path() {
  const res_path = await resourceDir();
  const db_path = await resolveResource('__data/database/database.db');
  const blank_db_path = await resolveResource('__data/database/db_blank.db');
  const db_sql = await resolveResource('__data/sql');
  let v = [
    {
      key: "RES_PATH",
      path: res_path
    },
    {
      key: "DB_PATH",
      path: db_path
    },
    {
      key: "BLANK_DB_PATH",
      path: blank_db_path
    },
    {
      key: "DB_SQL",
      path: db_sql
    },
  ]
  await invoke("set_path_js", { v })
}

const handleMouse = (e) => {
  e.preventDefault();
}
set_path()
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

.el-aside {
  width: 130px;
  background-color: #bbe6d6;
  color: rgb(8, 7, 7);
  height: 100vh;

  ul {
    background-color: #bbe6d6;
  }

  img {
    height: 32px;
    width: 32px;
    padding-left: 10px;
  }
}

.el-main {
  background-color: #d9f1e8;
  color: rgb(14, 13, 13);
  height: 100vh;
}

.aside_logo {
  display: flex;
  height: 60px;
  align-items: center;
  cursor: pointer;

  span {
    font-size: 16px;
    font-weight: 800;
    padding-left: 5px;
  }
}

.el-menu {
  font-weight: 900;
  border-style: none;
}

.el-menu-item {
  background-color: #bae8d5 !important;
  font-weight: 800;
  height: 100%;
}

.el-menu-item.is-active {
  background-color: #d9f1e8 !important;
  border-radius: 30px 0px 0px 30px;
}

html.dark {
  .el-header {
    background-color: rgb(13, 17, 23);
    color: #d9f1e8;
  }

  .el-aside {
    background-color: rgb(13, 17, 23);
    color: rgb(209, 191, 191);

    ul {
      background-color: rgb(13, 17, 23);
    }
  }

  .el-main {
    background-color: rgb(1, 4, 9);
    color: rgb(216, 203, 203);
  }

  .el-menu-item {
    background-color: rgb(13, 17, 23) !important;
  }

  .el-menu-item.is-active {
    background-color: rgb(1, 4, 9) !important;
  }
}
</style>
