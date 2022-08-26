<template>
  <span>医院</span>
  <el-divider></el-divider>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="名称" prop="name">
        <el-input v-model="queryParams.name" placeholder="请输入名称" clearable @keyup.enter="getList" />
      </el-form-item>

      <el-form-item label="状态" prop="status">
        <el-select v-model="queryParams.status" placeholder="状态" clearable>
          <el-option v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>

      <el-form-item>
        <el-button type="primary" :icon="Search" @click="getList">搜索</el-button>
        <el-button :icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8" style="height: 35px">
      <el-col :span="1.5">
        <el-button type="primary" plain :icon="Plus" @click="handleAdd">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="success" plain :icon="Edit" :disabled="single" @click="handleUpdate">修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain :icon="Delete" :disabled="multiple" @click="handleDelete">删除</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table border v-loading="loading" :data="dataList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="#" align="center" width="50" type="index" />
      <el-table-column label="医院名称" align="center" :show-overflow-tooltip="true">
        <template #default="scope">
          <el-link type="success" @click="goToData(scope.row.id, scope.row.name)">{{ scope.row.name }}
          </el-link>
        </template>
      </el-table-column>

      <el-table-column label="位置" align="center" width="150" show-overflow-tooltip>
        <template #default="scope">
          <span>{{ scope.row.province }}-{{ scope.row.city }}</span>
        </template>
      </el-table-column>
      <el-table-column label="详细地址" align="center" prop="location" show-overflow-tooltip />
      <el-table-column label="排序" align="center" prop="sort" width="100" />
      <el-table-column label="备注" align="center" prop="remark" />
      <el-table-column label="状态" align="center" prop="status" width="100">
        <template #default="scope">
          <dict-tag :options="sys_normal_disable" :value="scope.row.status" />
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center" width="180" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-link type="success" :icon="Edit" @click="handleUpdate(scope.row)">修改</el-link>
          <el-link type="danger" :icon="Delete" @click="handleDelete(scope.row)">删除</el-link>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.page_num"
      v-model:limit="queryParams.page_size" @pagination="getList" />
  </div>
  <!-- 添加或修改对话框 -->
  <el-dialog :title="title" v-model="open" width="400px" append-to-body>
    <el-form ref="formRef" :model="form" :rules="rules" label-width="80px">
      <el-form-item label="省份名称" prop="province">
        <el-input v-model="form.province" placeholder="请输入省份名称" clearable />
      </el-form-item>
      <el-form-item label="城市名称" prop="city">
        <el-input v-model="form.city" placeholder="请输入城市名称" clearable />
      </el-form-item>
      <el-form-item label="医院名称" prop="name">
        <el-input v-model="form.name" placeholder="请输入医院名称" clearable />
      </el-form-item>

      <el-form-item label="详细地址" prop="location">
        <el-input type="textarea" v-model="form.location" placeholder="请输入具体地址" clearable />
      </el-form-item>


      <el-form-item label="备注" prop="remark">
        <el-input type="textarea" v-model="form.remark" placeholder="请输入备注" clearable />
      </el-form-item>

      <el-form-item label="排序" prop="sort">
        <el-input-number v-model="form.sort" controls-position="right" :min="0" :max="99999" />
      </el-form-item>

      <el-form-item label="状态" prop="status">
        <el-radio-group v-model="form.status">
          <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.value">{{ dict.label }}
          </el-radio>
        </el-radio-group>
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button type="primary" @click="submitForm">确 定</el-button>
        <el-button @click="cancel">取 消</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup name="Hospital">
import { getCurrentInstance, ref, toRefs, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { Plus, Edit, Delete, Search, Refresh } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from "element-plus"
// import { getAllProvince } from '@/api/dm/Province';
// import { listCity } from '@/api/dm/City';
// import {
//   listHospital,
//   addHospital,
//   delHospital,
//   getHospital,
//   updateHospital,
// } from '@/api/dm/Hospital';

import Instrument from './instrument.vue';

const { proxy } = getCurrentInstance();
const { sys_normal_disable } = proxy.useDict('sys_normal_disable');

const dataList = ref([]);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const names = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref('');
const citiesMap = ref({});




const data = reactive({
  form: {},
  queryParams: {
    page_num: 1,
    page_size: 10,
    name: undefined,
    status: undefined,
  },
  rules: {
    name: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
    province: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
    city: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
    sort: [{ required: true, message: '排序不能为空', trigger: 'blur' }],
  },
});

const { queryParams, form, rules } = toRefs(data);

/** 查询岗位列表 */
async function getList() {
  loading.value = true;
  const qp = {
    pageParams: {
      page_num: queryParams.value.page_num,
      page_size: queryParams.value.page_size
    },
    req: {
      name: queryParams.value.name,
      status: queryParams.value.status,
    }
  }
  const [res, msg] = await invoke("get_hospital_list", { ...qp })
  if (msg == null) {
    dataList.value = res.list;
    total.value = res.total;
  }
  else {
    ElMessage.error(msg)
  }
  loading.value = false;
}

/** 取消按钮 */
function cancel() {
  open.value = false;
  reset();
}
/** 表单重置 */
function reset() {
  form.value = {
    id: undefined,
    name: undefined,
    sort: 0,
    status: '1',
  };
  proxy.resetForm('formRef');
}




/** 重置按钮操作 */
const resetQuery = async () => {
  proxy.resetForm('queryRef');
  queryParams.value.page_num = 1;
  await getList();
};
/** 多选框选中数据 */
function handleSelectionChange(selection) {
  ids.value = selection.map((item) => item.id);
  names.value = selection.map((item) => item.name);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}
/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = '添加医院';
}
/** 修改按钮操作 */
async function handleUpdate(row) {
  reset();
  const id = row.id || ids.value[0];
  const [data, msg] = await invoke("get_hospital_by_id", { id })
  if (msg == null) {
    form.value = data;
    open.value = true;
    title.value = '修改医院';
  } else {
    ElMessage.error(msg)
  }
}
/** 提交按钮 */
function submitForm() {
  proxy.$refs['formRef'].validate(async (valid) => {
    if (valid) {
      if (form.value.id != undefined) {
        let [_res, msg] = await invoke("edit_hospital", { req: form.value });
        if (msg == null) {
          ElMessage.success('修改成功')
          open.value = false;
          getList();
        } else {
          ElMessage.error(msg)
        }
      } else {
        let [_res, msg] = await invoke("add_hospital", { req: form.value });
        if (msg == null) {
          ElMessage.success('新增成功')
          open.value = false;
          getList();
        } else {
          ElMessage.error(msg)
        }

      }
    }
  });
}
/** 删除按钮操作 */
function handleDelete(row) {
  const id_s = row.id ? [row.id] : ids.value;
  const name_s = row.id ? row.name : names.value;
  ElMessageBox
    .confirm('是否确认删除岗位编号为"' + name_s + '"的数据项？')
    .then(async () => {
      let [res, msg] = await invoke("delete_hospital", { ids: id_s });
      if (msg == null) {
        ElMessage.success('删除成功')
        getList();
      } else {
        ElMessage.error(msg)
      }

    }).catch(() => { });
}
//  数据页面跳转
const goToData = (id, name) => {
  proxy.$router.push({path:"/hospital_ins",query: {id,name}});
}

getList();
</script>

<style lang="scss" scoped>
.el-link {
  margin-right: 8px;
}
</style>
