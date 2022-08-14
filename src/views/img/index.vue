<template>
  <div>
    <span>{{t('img.title')}}</span>
    <el-divider></el-divider>
    <el-form ref="queryRef" label-width="100px">
      <!-- 功能选择 -->
      <el-form-item :label="t('img.function')">
        <el-radio-group v-model="activeTab">
          <el-radio label="compressImg">{{t('img.compressImg')}}</el-radio>
          <el-radio label="deleteImg">{{t('img.delete_err_img')}}</el-radio>
        </el-radio-group>
      </el-form-item>
      <!-- 原始图片目录 -->
      <el-form-item :label="t('img.imgDir')">
        <el-input
          v-model="origin_dir"
          :placeholder="t('img.imgDir_placeholder')"
          clearable
          @click="open_dir_dialog"
        />
      </el-form-item>
      <!-- 开始处理按钮 -->
      <el-form-item :label="t('img.program')">
        <el-form-item>
          <el-button type="primary" :icon="Promotion" @click="run_program">
            {{t('img.run')}}</el-button
          >
          <el-button :icon="Refresh" @click="clear_logs">{{t('img.clearLog')}}</el-button>
        </el-form-item>
      </el-form-item>
      <!-- 压缩图片时保存目录 -->
      <el-form-item :label="t('img.log')">
        <el-input
          v-model="logs"
          :placeholder="t('img.log_placeholder')"
          type="textarea"
          :rows="10"
          clearable
        />
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup>
import { open } from '@tauri-apps/api/dialog';
import { Refresh, Promotion } from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/tauri';
import { getCurrentInstance } from 'vue';

import i18n from '@/locals'; 
const { t } = i18n.global;


const { proxy } = getCurrentInstance();

// 激活Tab
const activeTab = ref('compressImg');
// 日志
const logs = ref('');
// 需处理文件目录
const origin_dir = ref('');
//  日志刷新
const timer = ref(null);
// 任务完成标志
const task_done = ref(false);

onDeactivated(() => {
  stop_timer();
});

// 清除日志
const clear_logs = () => {
  logs.value = '';
};
// 弹出文件夹选择框
const open_dir_dialog = async (v) => {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  if (selected === null) {
    return;
  } else {
    origin_dir.value = selected;
  }
};
// 运行程序
const run_program = async () => {
  fresh_img_logs();
  if (activeTab.value == 'compressImg') {
    await invoke('compress_img', { dir: origin_dir.value });
  }
  if (activeTab.value == 'deleteImg') {
    await invoke('delete_err_img', { dir: origin_dir.value });
  }
  task_done.value = true;
};
// 获取日志
const get_logs = async () => {
  const log = await invoke('get_img_log');
  if (log.length == 0 && task_done.value) {
    stop_timer();
  }
  logs.value += log;
};
// 定时获取日志
const fresh_img_logs = () => {
  proxy.timer = setInterval(() => {
    setTimeout(get_logs, 0);
  }, 500);
};

const stop_timer = () => {
  clearInterval(proxy.timer);
  timer.value = null;
};
</script>
