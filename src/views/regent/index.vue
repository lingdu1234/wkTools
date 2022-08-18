<template>
  <div style="display: flex; justify-content: center; align-items: center">
    <span>项目</span>
    <el-radio-group v-model="activeGroup" style="margin:0 auto -5px;">
      <el-radio-button label="labTest">项目管理</el-radio-button>
      <el-radio-button label="testComp">项目对比</el-radio-button>
    </el-radio-group>
    <span></span>
  </div>
  <el-divider></el-divider>
  <div class="app-container">

    <testCompVue v-if="activeGroup === 'testComp'" />
    <div v-if="activeGroup === 'labTest'">
      <div style="display: flex; justify-content: center; align-items: center">
        <el-radio-group v-model="activeTestGroup" style="margin: -15px auto 10px;" @change="getList">
          <el-radio-button label="ALL">ALL</el-radio-button>
          <el-radio-button v-for="dict in mc_test_group" :key="dict.label" :label="dict.value"></el-radio-button>
        </el-radio-group>
      </div>

      <el-row :gutter="10" class="mb8" style="height: 35px">
        <el-col :span="1.5">
          <el-button type="primary" plain :icon="Plus" @click="handleAdd">新增项目</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="success" plain :icon="Edit" @click="handleRelation">新增关联</el-button>
        </el-col>
      </el-row>
      <el-table v-loading="loading" :data="dataList">
        <!-- :height="table_height" -->
        <el-table-column :index="(index) => index" align="center" label="#" type="index" />
        <el-table-column label="简称" align="center" prop="name_en" />
        <el-table-column label="code" align="center" prop="code" />
        <el-table-column label="中文" align="center" prop="name_cn" />
        <el-table-column label="排序" align="center" prop="order" />
        <el-table-column align="center" label="备注">
          <template #default="scope">
            <el-popover placement="top-start" :title="scope.row.name_en" trigger="hover" width="300px">
              <template #reference>
                <span width="100">
                  {{
                      scope.row.remark.length > 10
                        ? scope.row.remark.slice(0, 10) + '…'
                        : scope.row.remark
                  }}</span>
              </template>
              <div style="width: 100">{{ scope.row.remark }}</div>
            </el-popover>
          </template>
        </el-table-column>
        <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-link type="success" :icon="Edit" @click="handleUpdate(scope.row)">修改</el-link>
            <el-link type="danger" :icon="Delete" @click="handleDelete(scope.row)">删除</el-link>
          </template>
        </el-table-column>
      </el-table>
      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.page_num"
        v-model:limit="queryParams.page_size" @pagination="getList" />
    </div>
    <!-- 添加或修改岗位对话框 -->
    <el-dialog :title="title" v-model="open" width="500px" append-to-body>
      <el-form ref="formRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="项目代码" prop="code">
          <el-input v-model="form.code" placeholder="请输入项目代码" />
        </el-form-item>
        <el-form-item label="项目简称" prop="name_en">
          <el-input v-model="form.name_en" placeholder="请输入项目简称" />
        </el-form-item>
        <el-form-item label="项目中文" prop="name_cn">
          <el-input v-model="form.name_cn" placeholder="请输入项目中文" />
        </el-form-item>
        <el-form-item label="排序" prop="order">
          <el-input v-model="form.order" placeholder="请输入排序" />
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          <el-input v-model="form.remark" placeholder="请输入备注" type="textarea" />
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

  <!-- 添加或修改项目组关联项目对话框 -->
  <el-dialog :title="title" v-model="testRelatedDialogOpen" append-to-body width="350px">
    <el-form ref="relationFormRef" :model="relationForm" :rules="relationFormRules" label-width="80px">
      <el-form-item label="测试组" prop="test_group">
        <el-select v-model="relationForm.test_group" placeholder="状态" clearable>
          <el-option v-for="dict in mc_test_group" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item label="项目名称" prop="test_code">
        <el-select v-model="relationForm.test_code" placeholder="项目名称" @change="handleTestOptionChanged">
          <el-option v-for="dict in testOptions" :key="dict.key" :label="dict.key + ' -- ' + dict.value"
            :value="dict.key"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="组内排序" prop="test_order">
        <el-input v-model="relationForm.test_order" placeholder="请输入组内排序" />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button type="primary" @click="submitRelationForm">确 定</el-button>
        <el-button @click="cancel">取 消</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup name="LabTest">
import { getCurrentInstance, ref, toRefs, reactive, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { Plus, Edit, Delete } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from "element-plus"
import testCompVue from './testComp.vue';

const { proxy } = getCurrentInstance();

import i18n from '@/locals';
const { t } = i18n.global;

const { mc_test_group } = proxy.useDict('mc_test_group');

const dataList = ref([]);
const activeGroup = ref('labTest');
const activeTestGroup = ref('ALL');
const open = ref(false);
const testRelatedDialogOpen = ref(false);
const loading = ref(true);
const ids = ref([]);
const total = ref(0);
const title = ref('');
const testOptions = ref([]);
const testOptionsObj = ref({});



const data = reactive({
  form: {},
  relationForm: {},
  queryParams: {
    page_num: 1,
    page_size: 10,
    test_group: undefined,
  },
  rules: {
    code: [{ required: true, message: '代码不能为空', trigger: 'blur' }],
    name_en: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
    name_cn: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
    order: [{ required: true, message: '排序不能为空', trigger: 'blur' }],
  },
  relationFormRules: {
    test_group: [{ required: true, message: '代码不能为空', trigger: 'blur' }],
    test_code: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
  },
});

const { queryParams, form, relationForm, rules, relationFormRules } =
  toRefs(data);

/** 查询岗位列表 */
async function getList() {
  loading.value = true;
  if (activeTestGroup.value == 'ALL') {
    queryParams.value.test_group = undefined;
  } else {
    queryParams.value.test_group = activeTestGroup.value;
  }
  const qp = {
    pageParams: {
      page_num: queryParams.value.page_num,
      page_size: queryParams.value.page_size
    },
    req: {
      test_group: queryParams.value.test_group,
    }
  }
  const [data, msg] = await invoke("get_regent_list", qp)
  if (msg == null) {
    dataList.value = data.list;
    total.value = data.total;
  } else {
    ElMessage.error(msg)
  }
  loading.value = false;
}

// 获取全部测试数
const getTestList = async () => {
  loading.value = true;
  const qp = {
    pageParams: {
      page_num: queryParams.value.page_num,
      page_size: Number.MAX_SAFE_INTEGER,
    },
    req: {
      test_group: queryParams.value.test_group,
    }
  }
  const [response, _msg] = await invoke("get_regent_list", qp)
  const _testOptions = [];
  const _testOptionsObj = {};
  response.list.forEach((item) => {
    const _test = {
      key: item.code,
      value: item.name_en,
    };
    _testOptions.push(_test);
    _testOptionsObj[item.code] = {
      name: item.name_en,
      oder: item.order,
    };
  });
  testOptions.value = _testOptions;
  testOptionsObj.value = _testOptionsObj;
  loading.value = false;
};
const handleTestOptionChanged = (v) => {
  relationForm.value.test_name = testOptionsObj.value[v].name;
  // relationForm.value.test_order = testOptionsObj.value[v].oder;
};
/** 取消按钮 */
function cancel() {
  open.value = false;
  testRelatedDialogOpen.value = false;
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
  proxy.resetForm('relationFormRef');
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = '添加项目';
}
function handleRelation() {
  reset();
  testRelatedDialogOpen.value = true;
  title.value = '添加项目关联';
}
/** 修改按钮操作 */
async function handleUpdate(row) {
  reset();
  const id = row.id || ids.value[0];
  const [data, msg] = await invoke("get_regent_by_id", { id });
  if (msg == null) {
    form.value = data;
    open.value = true;
    title.value = '修改项目';
  } else {
    ElMessage.error(msg)
  }

}
/** 提交按钮 */
function submitForm() {
  proxy.$refs['formRef'].validate(async (valid) => {
    if (valid) {
      if (form.value.id != undefined) {
        let [_, msg] = await invoke("edit_regent", { req: form.value });
        if (msg == null) {
          ElMessage.success('修改成功')
          open.value = false;
          getList();
        } else {
          ElMessage.error(msg)
        }
      } else {
        let [_, msg] = await invoke("add_regent", { req: form.value });
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
/** 项目关联提交按钮 */
function submitRelationForm() {
  proxy.$refs['relationFormRef'].validate(async (valid) => {
    if (valid) {
      const [_, msg] = await invoke("add_regent_group", { req: relationForm.value });
      if (msg == null) {
        testRelatedDialogOpen.value = false;
        getList();
        ElMessage.success('修改成功');
      } else {
        ElMessage.error(msg);

      }
    }
  });
}
/** 删除按钮操作 */
function handleDelete(row) {
  if (activeTestGroup.value == 'ALL') {
    ElMessageBox
      .confirm('是否确认删除"' + row.name_en + '"的数据项？')
      .then(async () => {
        const [data, msg] = await invoke("delete_regent", { id: row.id, code: row.code });
        if (msg == null) {
          ElMessage.success(data)
          getList();
        } else {
          ElMessage.error(msg)
        }
      })
      .catch(() => { });
  } else {
    ElMessageBox
      .confirm(
        '是否确认删除"' +
        row.name_en +
        '与' +
        activeTestGroup.value +
        '"关联关系？'
      )
      .then(async () => {
        const [_, msg] = await invoke("delete_regent_group", {
          req: {
            test_name: row.name_en,
            test_group: activeTestGroup.value,
          }
        });
        getList();
        ElMessage.success('关联关系删除成功');
      })
      .catch(() => { });
  }
}

getList();
getTestList();
</script>
<style scoped>
.el-link {
  margin-right: 8px;
}
</style>