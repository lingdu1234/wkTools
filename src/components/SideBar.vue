<template>
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
        <span>{{ t(menu.meta.title) }}</span>
      </el-menu-item>
    </el-menu>
  </el-aside>
</template>

<script setup>
import { constantRoutes as menus } from '@/router';
import { getCurrentInstance } from 'vue';

import i18n from '@/locals'

const { t } = i18n.global


const { proxy } = getCurrentInstance();

const emits = defineEmits(['update:route_title']);

// 激活菜单
const activePath = ref(null);

onMounted(() => {
  activePath.value = localStorage.getItem("sfsafasfsa") || "/index"
  proxy.$router.push(activePath.value);
})

function handleMenuChange(index, _indexPath) {
  activePath.value = index;
  proxy.$router.push(index);
}
function getMenu(index, title) {
  localStorage.setItem('sfsafasfsa', index)
  localStorage.setItem('wfw3t3t32t', title)
   emits("update:route_title", title);
}

function gotoAdminHome() {
  proxy.$router.push('/index');
  activePath.value = '/index';
}
</script>


<style lang="scss" scoped>
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
