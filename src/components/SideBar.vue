<template>
  <el-aside>
    <div>
      <div class="aside_logo" @click="gotoAdminHome">
        <el-badge v-if="appStore.updatable" value="new">
          <img src="@/assets/logo2.png" />
        </el-badge>
        <img v-else src="@/assets/logo2.png" />
        <span>WkTools</span>
      </div>
      <el-menu active-text-color="#5352ed" :default-active="routeInfo.path" unique-opened @select="handleMenuChange">
        <el-menu-item v-for="menu in menus" :index="menu.path" :key="menu.path"
          @click="getMenu(menu.path, menu.meta.title)">
          <el-icon>
            <SvgIcon :name="menu.meta.icon"></SvgIcon>
          </el-icon>
          <span>{{ t(menu.meta.title) }}</span>
        </el-menu-item>
      </el-menu>
    </div>
  </el-aside>

  <Updater :is_show="is_show_dialog" @closeUpdateDialog="closeUpdateDialog" />
</template>

<script setup>
import { constantRoutes as menus } from '@/router';
import useAppStore from "@/store/modules/app"

import i18n from '@/locals'

const appStore = useAppStore();

const is_show_dialog = ref(false)

const { t } = i18n.global

const { proxy } = getCurrentInstance();

// 激活菜单
const routeInfo = appStore.routeInfo;

onMounted(() => {
  proxy.$router.push(routeInfo.path);
})

function handleMenuChange(index) {
  proxy.$router.push(index);
}
function getMenu(index, title) {
  appStore.setRouteInfo({
    title: title,
    path: index
  })

}

function gotoAdminHome() {
  if (appStore.updatable) {
    is_show_dialog.value = true
  } else {
    proxy.$router.push('/index');
  }

}

const closeUpdateDialog = () => {
  is_show_dialog.value = false
}
</script>


<style lang="scss" scoped>
.el-aside {
  width: 130px;
  background-color: #bbe6d6;
  color: rgb(8, 7, 7);
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: space-between;

  ul {
    background-color: #bbe6d6;
  }

  img {
    height: 32px;
    width: 32px;
    padding-left: 10px;
  }
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

.operatorClassA {
  width: 100px;
  display: flex;
  justify-content: space-around;
}

html.dark {

  .el-aside {
    background-color: rgb(13, 17, 23);
    color: rgb(209, 191, 191);

    ul {
      background-color: rgb(13, 17, 23);
    }
  }

  .el-menu-item {
    background-color: rgb(13, 17, 23) !important;
  }

  .el-menu-item.is-active {
    background-color: rgb(1, 4, 9) !important;
  }
}
</style>
