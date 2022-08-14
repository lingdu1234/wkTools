<template>
  <div>
    <span>{{t("about.title")}}</span>
    <div class="logo">
      <img src="@/assets/logo2.png" />
    </div>
    <div class="updater">
      <el-button type="primary" @click="check_update">{{t("about.checkUpdate")}}</el-button>
    </div>
  </div>
  <el-divider></el-divider>
  <div class="content">
    <p>{{t("about.p01")}}</p>
    <p>{{t("about.p02")}}</p>
    <p>
      {{t("about.p03")}}
    </p>
    <el-divider></el-divider>
    <p>{{t("about.aboutProgram")}}</p>
    <p>
      <span>{{t("about.programName")}}：</span><span>{{ v.appName }}</span>
    </p>
    <p>
      <span>{{t("about.programVersion")}}：</span><span>{{ v.appVersion }}</span>
    </p>
    <p>
      <a href="https://tauri.app/" target="_blank">tauri</a
      ><span>{{t("about.version")}}：{{ v.tauriVersion }}</span>
    </p>
  </div>
</template>

<script setup>
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

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

const check_update = async () => {
  try {
  const { shouldUpdate, manifest } = await checkUpdate()
  console.log('first', shouldUpdate)
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
  max-width: 550px;
  font-size: 18px;
  margin: 0 auto;
}
</style>
