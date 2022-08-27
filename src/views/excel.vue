<template>
  <div>
    <span>{{ t('excel.title') }}</span>
    <el-divider></el-divider>
    <el-form ref="queryRef" label-width="100px">
      <!-- 功能选择 -->
      <el-form-item :label="t('img.function')">
        <el-radio-group v-model="activeTab">
          <el-radio label="excel">{{ t('excel.export') }}</el-radio>
          <el-radio label="excel_dir">{{ t('excel.export_dir') }}</el-radio>
        </el-radio-group>
      </el-form-item>
      <!-- 原始目录 -->
      <el-form-item :label="t('excel.excelDir')">
        <el-input v-model="origin_dir" :placeholder="t('excel.excelDir_placeholder')" clearable
          @click="open_dir_dialog('excel')" />
      </el-form-item>
      <!-- 原始图片目录 -->
      <el-form-item :label="t('img.exceTolDir')">
        <el-input v-model="to_dir" :placeholder="t('excel.excel_save_placeholder')" clearable
          @click="open_dir_dialog(('excel_dir'))" />
      </el-form-item>
      <!-- 开始处理按钮 -->
      <el-form-item :label="t('img.program')">
        <el-form-item style="width:500px">
          <el-button type="primary" :icon="Promotion" @click="run_program">
            {{ t('img.run') }}</el-button>
          <el-button :icon="Refresh" @click="clear_logs">{{ t('img.clearLog') }}</el-button>
        </el-form-item>
      </el-form-item>
      <!-- 压缩图片时保存目录 -->
      <el-form-item :label="t('img.log')">
        <el-input id="textarea_log" v-model="logs" :placeholder="t('img.log_placeholder')" type="textarea" :rows="10"
          clearable />
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
const activeTab = ref('excel');
// 日志
const logs = ref('');
// 需处理文件目录
const origin_dir = ref('');
// 导出保存目录
const to_dir = ref("")
//  日志刷新
const timer = ref(null);
// 任务完成标志
const task_done = ref(false);

onDeactivated(() => {
  stop_timer();
});

const log_scroll = () => {
  nextTick(() => {
    const textarea = document.getElementById('textarea_log');
    console.log('textarea.scrollHeight :>> ', textarea.scrollHeight);
    textarea.scrollTop = textarea.scrollHeight;
  })
}

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
    if (v === 'excel') {
      origin_dir.value = selected;
    } else {
      to_dir.value = selected;
    }
  }
};
// 运行程序
const run_program = async () => {
  console.log(origin_dir.value, to_dir.value)
  fresh_logs();
  if (activeTab.value == 'excel') {
    await invoke('export_excel', { dir: origin_dir.value, toDir: to_dir.value, isWithDir: false });
  }
  if (activeTab.value == 'excel_dir') {
    await invoke('export_excel', { dir: origin_dir.value, toDir: to_dir.value, isWithDir: true });
  }
  task_done.value = true;
};
// 获取日志
const get_logs = async () => {
  const log = await invoke('get_log');
  if (log.length == 0 && task_done.value) {
    stop_timer();
    return
  }
  logs.value += log;
  log_scroll()

};
// 定时获取日志
const fresh_logs = () => {
  proxy.timer = setInterval(() => {
    setTimeout(get_logs, 0);
  }, 500);
};

const stop_timer = () => {
  clearInterval(proxy.timer);
  timer.value = null;
};
</script>
