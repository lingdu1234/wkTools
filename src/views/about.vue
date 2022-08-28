<template>
  <div>
    <span>{{ t("about.title") }}</span>
    <div class="logo">
      <img src="@/assets/logo2.png" />
    </div>
    <div class="updater">
      <el-button type="primary" @click="check_update">{{ t("about.checkUpdate") }}</el-button>
    </div>
  </div>
  <el-divider></el-divider>
  <div class="content">
    <el-divider content-position="left">{{ t("about.desc") }}</el-divider>
    <p>{{ t("about.p01") }}</p>
    <p>{{ t("about.p02") }}</p>
    <p>
      {{ t("about.p03") }}
    </p>
    <p>{{ t("about.contact") }}</p>
    <el-divider content-position="left">{{ t("about.aboutProgram") }}</el-divider>
    <table class="pure-table pure-table-bordered" style="margin:0 auto;">
      <tbody>
        <tr>
          <td>{{ t("about.programName") }}</td>
          <td>{{ v.appName }}</td>
          <td>{{ t("about.programVersion") }}</td>
          <td>{{ v.appVersion }}</td>
        </tr>
        <tr class="pure-table-odd">
          <td>{{ t("about.mainFrame") }}</td>
          <td><a href="https://tauri.app/" target="_blank">tauri</a></td>
          <td>{{ t("about.edition") }}</td>
          <td>{{ v.tauriVersion }}</td>
        </tr>
        <tr>
          <td><a href="https://tokio.rs/" target="_blank">Tokio</a></td>
          <td><a href="https://www.sea-ql.org/" target="_blank">SeaOrm</a></td>
          <td><a href="https://kazupon.github.io/vue-i18n/" target="_blank">Vue I18n</a></td>
          <td><a href="https://crates.io/crates/image" target="_blank">image</a></td>
        </tr>
        <tr class="pure-table-odd">
          <td><a href="https://cn.vuejs.org/" target="_blank">VUE</a></td>
          <td><a href="https://vitejs.cn/" target="_blank">vite3</a></td>
          <td><a href="https://element-plus.org/" target="_blank">element-plus</a></td>
          <td><a href="https://sheetjs.com/" target="_blank">XLXS</a></td>
        </tr>
        <tr>
          <td>{{ t("app.dir") }}</td>
          <td colspan="3">{{ app_dir }}</td>
        </tr>
        <tr>
          <td>{{ t("app.log_dir") }}</td>
          <td colspan="3">{{ log_dir }}</td>
        </tr>

        <tr>
          <td>{{ t("app.config_dir") }}</td>
          <td colspan="3">{{ config_dir }}</td>
        </tr>
        <tr>
          <td>{{ t("app.res_dir") }}</td>
          <td colspan="3">{{ res_dir }}</td>
        </tr>
        <tr>
          <td>{{ t("app.download_dir") }}</td>
          <td colspan="3">{{ download_dir }}</td>
        </tr>
      </tbody>
    </table>
    <el-divider content-position="left">{{ t("about.aboutProgram") }}</el-divider>

  </div>
</template>

<script setup>
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import { configDir, appDir, resourceDir, downloadDir,logDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/tauri';


import i18n from '@/locals';
const { t } = i18n.global;

const app_dir = ref(null);
const log_dir = ref(null);
const config_dir = ref(null);
const res_dir = ref(null);
const download_dir = ref(null);

const v = ref({
  appName: '',
  tauriVersion: '',
  appVersion: '',
});
const get_base_info = async () => {
  v.value.appName = await getName();
  v.value.tauriVersion = await getTauriVersion();
  v.value.appVersion = await getVersion();
};

async function get_app_dir() {
  res_dir.value = await resourceDir();
  app_dir.value = await appDir();
  config_dir.value = await configDir();
  log_dir.value = await logDir();
  download_dir.value = await downloadDir();

}

const check_update = async () => {
  try {
    const { shouldUpdate, manifest } = await checkUpdate()
    if (shouldUpdate) {
      // display dialog
      await installUpdate()
      // install complete, restart app
      await relaunch()
    }
  } catch (error) {
    console.log(error)
  }
}
get_base_info();
get_app_dir();
</script>


<style lang="scss" scoped>
.logo {
  width: 100px;
  height: 100px;
  margin: 10px auto 10px auto;
  display: flex;
}

.updater {
  width: 100px;
  margin: 10px auto 0px auto;
  display: flex;
}

.content {
  max-width: 1000px;
  font-size: 18px;
  margin: 0 auto;
}

table {
  border-collapse: collapse;
  border-spacing: 0;
  margin: 1px;
  width: 90%;
}

td,
th {
  padding: 0;
  text-align: center;
}

.pure-table {
  border-collapse: collapse;
  border-spacing: 0;
  empty-cells: show;
  border: 1px solid #cbcbcb;
}

.pure-table caption {
  color: #000;
  font: italic 55%/1 arial, sans-serif;
  padding: 1em 0;
  text-align: center;
}

.pure-table td,
.pure-table th {
  border-left: 1px solid #cbcbcb;
  border-width: 0 0 0 1px;
  font-size: inherit;
  margin: 0;
  overflow: visible;
  padding: 0.5em 1em;
}

.pure-table thead {
  background-color: #e0e0e0;
  color: #000;
  text-align: left;
  vertical-align: bottom;
}

.pure-table td {
  max-width: 200px;
  min-width: 100px;
  width: 25%;
  background-color: transparent;
}

.pure-table-bordered td {
  border-bottom: 1px solid #cbcbcb;
}

.pure-table-bordered tbody>tr:last-child>td {
  border-bottom-width: 0;
}

.pure-table-odd td {
  background-color: #dbbdbd64;
}

html.dark {
  .pure-table-odd td {
    background-color: #262323d3;
  }

  .pure-table th {
    background-color: rgba(34, 31, 31, 0.999);
    color: white;
  }
}
</style>
