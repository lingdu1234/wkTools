<template>
  <div>
    <span>程序设置</span>
    <el-divider></el-divider>
    <div class="setting">
      <!-- 主题颜色 -->
      <div class="setting_item">
        <div class="setting_content">
          <span>主题颜色</span>
          <el-button :type="isDark ? 'info' : 'success'" @click="toggleDark()">
            <el-icon>
              <SvgIcon :name="isDark ? 'moon' : 'sun'"></SvgIcon>
            </el-icon>
            {{ isDark ? '黑暗模式' : '明亮模式' }}
          </el-button>
        </div>
        <el-divider class="setting_divider" />
      </div>
      <!-- 语言设置 -->
      <div class="setting_item">
        <div class="setting_content">
          <span>语言</span>
          <el-button v-if="locale === 'zh'" type="success" @click="changeLang('en')">
            <el-icon>
              <SvgIcon name="enDark"></SvgIcon>
            </el-icon>
            English
          </el-button>
          <el-button v-else type="success" @click="changeLang('zh')">
            <el-icon>
              <SvgIcon name="zhDark"></SvgIcon>
            </el-icon>
            简体中文
          </el-button>
        </div>
        <el-divider class="setting_divider" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { useDark, useToggle } from '@vueuse/core';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { useI18n } from 'vue-i18n';
import i18n from '@/locals'; // 当前语言
const { locale } = useI18n();

const isDark = useDark()

const toggleDark = useToggle(isDark);


onMounted(async () => {
  const v = localStorage.getItem('lang') || "zh";
  invoke('set_lang', { lang: v });
  window.onresize = () => {
    // get_window_height()
  };
})

// 切换语言
async function changeLang(v) {
  locale.value = v;
  await invoke('set_lang', { lang: v });
  localStorage.setItem('lang', v);
  window.location.reload();
}

</script>
<style lang="scss" scoped>
.setting {
  width: 100%;


  .setting_item {
    width: 80%;
    max-width: 650px;
    min-width: 500px;
    margin: 0 auto;
    align-items: center;

    .setting_content {
      width: 100%;
      max-width: 600px;
      min-width: 450px;
      margin: 0 auto;
      display: flex;
      align-items: center;
      justify-content: space-between;

      span {
        width: 70px;
        font-weight: bold;
        text-align: end;
      }
    }
  }
}

.setting_divider {
  margin: 12px 0;
}
</style>
