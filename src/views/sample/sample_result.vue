<template>
<div class="setting_content">
  <span>样本结果:     {{sample_code}}</span>
<el-button style="margin-right:30px" type="warning" plain :icon="Close" @click="handleClose">关闭</el-button>
</div>
  
  <el-divider></el-divider>
  <div style="margin-left: 20px; margin-right: 20px;margin-bottom: 30px;">
    <el-table border v-loading="loading" :data="dataList">
      <el-table-column label="#" align="center" width="50" type="index" :index="(index) => index" />
      <el-table-column label="项目组" align="center" width="100" prop="test_group" />
      <el-table-column label="项目id" align="center" prop="test_id" width="80" />
      <el-table-column label="项目" align="center" prop="test_name" width="120" />
      <el-table-column label="磁码数" align="center" prop="result_count" width="80" />
      <el-table-column label="信号值" align="center" prop="result_signal" />
      <el-table-column label="AI值" align="center" prop="result_ai" />
      <el-table-column label="定性" align="center" prop="result_index" />
      <el-table-column label="均值" align="center" prop="result_avg" />
      <el-table-column label="中位数" align="center" prop="result_med" />
      <el-table-column label="最小值" align="center" prop="result_min" />
      <el-table-column label="最大值" align="center" prop="result_max" />
      <el-table-column label="CV" align="center" prop="result_cv" />
    </el-table>
  </div>
</template>

<script setup name="McSampleResult">

import { invoke } from '@tauri-apps/api/tauri';
import { ElMessage } from 'element-plus';
import { useRoute } from 'vue-router';
import { Close } from '@element-plus/icons-vue';
import { getCurrentInstance } from 'vue';

const {proxy} = getCurrentInstance();


const dataList = ref([]);
const loading = ref(true);

const route  = useRoute()

const sample_code = ref("")

/** 查询列表 */
async function getList(id) {
  loading.value = true;
  const qp = {
    pageParams: {
      page_num: 1,
      page_size: Number.MAX_SAFE_INTEGER
    },
    req: {
      sample_id: id,
    }
  }
  const [response, msg] = await invoke("get_sample_result_list", { ...qp });
  if (msg !== null) {
    ElMessage.error(msg)
    return
  }
  dataList.value = response.list;
  sample_code.value = dataList.value.length > 0 ? dataList.value[0].sample_code:""
  loading.value = false;
}
/** 返回按钮操作 */
function handleClose() {
  proxy.$router.back();
}
getList(route.query && route.query.id);
</script>

<style lang="scss" scoped>
    .setting_content {
      width: 100%;
      margin: 0 auto;
      display: flex;
      align-items: center;
      justify-content: space-between;}
</style>