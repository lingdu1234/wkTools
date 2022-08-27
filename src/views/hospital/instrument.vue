<template>
  <span>{{ t("dict.instrument") }}</span>
  <el-divider></el-divider>
  <div class="app-container">
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
      <el-col :span="1.5">
        <el-button type="warning" plain :icon="Close" @click="handleClose">关闭</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="dataList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="#" align="center" width="50" type="index" />
      <el-table-column label="仪器" align="center" prop="name" width="100" show-overflow-tooltip />
      <el-table-column label="编号" align="center" prop="code" />
      <el-table-column label="SN" align="center" prop="sn" width="160" />
      <el-table-column label="位置" align="center" prop="location" show-overflow-tooltip />
      <el-table-column label="状态" align="center" prop="status">
        <template #default="scope">
          <dict-tag :options="sys_normal_disable" :value="scope.row.status" />
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center" width="150" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-link type="success" :icon="Edit" @click="handleUpdate(scope.row)">修改</el-link>
          <el-link type="danger" :icon="Delete" @click="handleDelete(scope.row)">删除</el-link>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.page_num"
      v-model:limit="queryParams.page_size" @pagination="getList" />

    <!-- 添加或修改参数配置对话框 -->
    <el-dialog :title="title" v-model="open" width="400px" append-to-body>
      <el-form ref="formRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="医院名称" prop="hospital_name">
          <el-input v-model="form.hospital_name" placeholder="请输入医院名称" />
        </el-form-item>
        <el-form-item label="仪器名称" prop="name">
          <el-select style="width: 300px" v-model="form.name" placeholder="请选择仪器名称">
            <el-option v-for="dict in instrumentNames" :key="dict.key" :label="dict.value" :value="dict.value">
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="编号" prop="code">
          <el-input v-model="form.code" placeholder="请输入编号" />
        </el-form-item>
        <el-form-item label="序列号" prop="sn">
          <el-input v-model="form.sn" placeholder="请输入序列号" />
        </el-form-item>
        <el-form-item label="仪器位置" prop="location">
          <el-input v-model="form.location" placeholder="请输入仪器位置" />
        </el-form-item>

        <el-form-item label="备注" prop="remark">
          <el-input v-model="form.remark" placeholder="请输入备注" />
        </el-form-item>
        <el-form-item label="状态">
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
  </div>
</template>

<script setup name="Instrument">
import { getCurrentInstance, ref, toRefs, reactive } from 'vue';
import { useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/tauri';

import { Plus, Edit, Delete, Close } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from "element-plus"


const { proxy } = getCurrentInstance();
const { sys_normal_disable, instrumentNames } = proxy.useDict(
  'sys_normal_disable',
  'instrumentNames'
);

import i18n from '@/locals';
const { t } = i18n.global;

const dataList = ref([]);
const open = ref(false);
const loading = ref(true);
const is_add = ref(true);
const ids = ref([]);
const names = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref('');
const route = useRoute();
// 数据标签回显样式
const listClassOptions = ref([
  { value: 'default', label: '默认' },
  { value: 'primary', label: '主要' },
  { value: 'success', label: '成功' },
  { value: 'info', label: '信息' },
  { value: 'warning', label: '警告' },
  { value: 'danger', label: '危险' },
]);

const data = reactive({
  form: {},
  queryParams: {
    page_num: 1,
    page_size: 10,
    hospital_id: undefined,
    hospital_name: undefined,
  },
  rules: {
    dict_label: [
      { required: true, message: '数据标签不能为空', trigger: 'blur' },
    ],
    dict_value: [
      { required: true, message: '数据键值不能为空', trigger: 'blur' },
    ],
    dict_sort: [
      { required: true, message: '数据顺序不能为空', trigger: 'blur' },
    ],
  },
});

const { queryParams, form, rules } = toRefs(data);

/** 查询字典类型详细 */
function getIns(id,name) {
  queryParams.value.hospital_id = id;
  queryParams.value.hospital_name = name;
  getList();
}
/** 查询字典数据列表 */
async function getList() {
  loading.value = true;
  const qp = {
    pageParams: {
      page_num: queryParams.value.page_num,
      page_size: queryParams.value.page_size
    },
    req: {
      hospital_id: queryParams.value.hospital_id,
    }
  }
  const [res, msg] = await invoke("get_instrument_list", { ...qp })
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
    hospital_id: undefined,
    hospital_name: undefined,
    sort: 0,
    status: '1',
  };
  proxy.resetForm('formRef');
}

/** 返回按钮操作 */
function handleClose() {
  proxy.$router.back();
}
/** 新增按钮操作 */
function handleAdd() {
  reset();
  form.value.hospital_name = queryParams.value.hospital_name;
  form.value.hospital_id = queryParams.value.hospital_id;
  open.value = true;
  title.value = '添加仪器数据';
console.log(form.value)
}
/** 多选框选中数据 */
function handleSelectionChange(selection) {
  ids.value = selection.map((item) => item.id);
  names.value = selection.map((item) => item.name);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}
/** 修改按钮操作 */
async function handleUpdate(row) {
  reset();
  is_add.value = false;
  const id = row.id || ids.value[0];

  const [data, msg] = await invoke("get_instrument_by_id", { id: id })
  if (msg == null) {
    form.value = data;
    open.value = true;
    title.value = '修改仪器数据';
  } else {
    ElMessage.error(msg)
  }

}
/** 提交按钮 */
function submitForm() {
  proxy.$refs['formRef'].validate(async (valid) => {
    if (valid) {
      if (form.value.dict_data_id != undefined) {
        let [_res, msg] = await invoke("edit_instrument", { req: form.value });
        if (msg == null) {
          ElMessage.success('修改成功')
          open.value = false;
          getList();
        } else {
          ElMessage.error(msg)
        }
      } else {
        let [_res, msg] = await invoke("add_instrument", { req: form.value });
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
    .confirm('是否确认删除字典编号为"' + name_s + '"的数据项？')
    .then(async () => {
      let [res, msg] = await invoke("delete_instrument", { ids: id_s });
      if (msg == null) {
        ElMessage.success('删除成功')
        getList();
      } else {
        ElMessage.error(msg)
      }

    }).catch(() => { });
}

getIns(route.query && route.query.id,route.query.name);
</script>
<style scoped>
.el-link {
  margin-right: 8px;
}
</style>