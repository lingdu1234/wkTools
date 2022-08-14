<template>
  <div>
    <span>程序设置</span>
    <el-divider></el-divider>
    <el-form ref="queryRef" label-width="100px">
      <!-- 功能选择 -->
      <el-form-item label="主题颜色">
        <el-button :type="isDark?'info':'success'" @click="toggleDark()">
          <el-icon>
            <SvgIcon :name="isDark?'moon':'sun'"></SvgIcon>
          </el-icon>

          {{ isDark ? '黑暗模式' : '明亮模式' }}
        </el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup>
import { useDark, useToggle } from '@vueuse/core';

const isDark = useDark()

const toggleDark = useToggle(isDark);

</script>
