<template>
  <div>
    <span>程序设置</span>
    <el-divider></el-divider>
    <el-form ref="queryRef" label-width="100px">
      <!-- 功能选择 -->
      <el-form-item label="主题颜色">
        <el-button :type="isDark ? 'info' : 'success'" @click="toggleDark()">
          <el-icon>
            <SvgIcon :name="isDark ? 'moon' : 'sun'"></SvgIcon>
          </el-icon>

          {{ isDark ? '黑暗模式' : '明亮模式' }}
        </el-button>
      </el-form-item>
      <el-button @click="get_data">点点</el-button>
      <span>{{data}}</span>
    </el-form>
  </div>
</template>

<script setup>
import { useDark, useToggle } from '@vueuse/core';
import { invoke } from '@tauri-apps/api/tauri';
const isDark = useDark()

const toggleDark = useToggle(isDark);
const data = ref(null)

const get_data = async () => {
  const v = await invoke('get_regent_group_by_test_group', { testGroup: "ANA-17" });
  data.value = v
  console.log(v)
}

</script>
