<template>
  <div class="shouye">
    <div class="logo">
      <img src="@/assets/logo2.png" />
    </div>
  </div>
  <div class="simg">
    <img src="@/assets/img/001.webp" :style="'max-height:' + window_h + 'px;max-width:' + window_w + 'px;'" />
  </div>
</template>

<script setup>
import { onBeforeUnmount } from 'vue';


const window_h = ref(null)
const window_w = ref(null)

onMounted(() => {
  getWindowResize()
  window.addEventListener('resize', getWindowResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', getWindowResize)
})

const getWindowResize = () => {
  window_h.value = document.documentElement.clientHeight - 250;
  window_w.value = document.documentElement.clientWidth - 200;
}
</script>


<style lang="scss" scoped>
.logo {
  width: 100px;
  height: 100px;
  margin: 10px auto 10px auto;
  display: flex;
}

.simg {
  img {
    margin: 10px auto 10px auto;
    display: block;
  }
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
</style>
