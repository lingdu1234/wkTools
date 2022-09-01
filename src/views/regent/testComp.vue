<template>
    <div>
        <div style="display: flex; justify-content: center; align-items: center">
            <el-radio-group v-model="activeGroup" style="margin: auto; padding: 10px" @change="tableShowChange">
                <el-radio-button label="A">方向A</el-radio-button>
                <el-radio-button label="B">方向B</el-radio-button>
            </el-radio-group>

        </div>
        <el-row :gutter="10" class="mb8" style="height: 35px">
            <el-col :span="1.5">
                <el-button type="success" plain :icon="Download" @click="handleExport">导出表格</el-button>
            </el-col>
        </el-row>
        <el-table v-loading="loading" :data="tableData" border stripe>
            <el-table-column align="center" label="#" type="index" />
            <el-table-column v-for="(v, i) in tableColumn" :key="i + v" :label="v" :prop="v" align="center">
            </el-table-column>
        </el-table>
    </div>
</template>

<script setup name="TestComp">
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';
// import { exportSingleListData2excel } from '@/utils/excelUtils';
import { ElMessage } from 'element-plus';
import { Download } from '@element-plus/icons-vue';

const dataList = ref([]);
const activeGroup = ref('A');
const listA = ref([]);
const listB = ref([]);
const excelData = ref([]);
const listMapA = ref([]);
const listMapB = ref([]);
const test_names = ref([]);
const test_groups = ref([]);
const tableData = ref([]);
const tableColumn = ref(['项目']);

const loading = ref(true);


const data = reactive({
    queryParams: {
        page_num: 1,
        page_size: Number.MAX_SAFE_INTEGER,
    },
});

const { queryParams } = toRefs(data);

function tableShowChange(v) {
    if (v == 'A') {
        tableData.value = listMapA.value;
        tableColumn.value = ['项目', ...test_names.value];
        excelData.value = listA.value;
    }

    if (v == 'B') {
        tableData.value = listMapB.value;
        tableColumn.value = ['项目', ...test_groups.value];
        excelData.value = listB.value;
    }
}

/** 查询岗位列表 */
async function getList() {
    loading.value = true;
    const qp = {
        pageParams: {
            page_num: queryParams.value.page_num,
            page_size: queryParams.value.page_size
        },
        req: {}
    }
    const [data, msg] = await invoke("get_regent_group", qp);
    if (msg == null) {
        dataList.value = data.list;
    } else {
        ElMessage.error(msg)
    }
    loading.value = false;

}

async function getBaseInfo() {
    dataList.value.forEach((e) => {
        if (!test_names.value.includes(e.test_name) && e.test_name != '本底') {
            test_names.value.push(e.test_name);
        }
        if (
            !test_groups.value.includes(e.test_group) &&
            e.test_group != '磁码Change'
        ) {
            test_groups.value.push(e.test_group);
        }
    });
}

async function getData() {
    listA.value[0] = ['项目', ...test_names.value];
    listB.value[0] = ['项目', ...test_groups.value];

    for (let index = 0; index < test_groups.value.length; index++) {
        let list = new Array(test_names.value.length).fill('');
        const e = test_groups.value[index];
        listMapA.value.push({
            '项目': e,
        });
        listA.value.push([e, ...list]);
    }
    test_names.value.forEach((e) => {
        let list = new Array(test_groups.value.length).fill('');
        listMapB.value.push({
            '项目': e,
        });
        listB.value.push([e, ...list]);
    });

    dataList.value.forEach((e) => {
        if (
            test_names.value.includes(e.test_name) &&
            test_groups.value.includes(e.test_group)
        ) {
            const indexG = test_groups.value.indexOf(e.test_group);
            const indexT = test_names.value.indexOf(e.test_name);
            listMapA.value[indexG][e.test_name] = '✓';
            listMapB.value[indexT][e.test_group] = '✓';
            listA.value[indexG + 1][indexT + 1] = '✓';
            listB.value[indexT + 1][indexG + 1] = '✓';
        }
    });
}

/** 导出按钮操作 */
async function handleExport() {
    const file_path = await save({
        defaultPath:"项目对比",
        filters: [{
            name:"",
            extensions: ['xlsx', 'xls']
        }]
    });
    await invoke("write_array_data_to_excel", { fileName: file_path, excelData: excelData.value })
    ElMessage.success("数据导出完成")
}

async function get() {
    await getList();
    await getBaseInfo();
    await getData();
    tableData.value = listMapA.value;
    tableColumn.value = ['项目', ...test_names.value];
    excelData.value = listA.value;
}

get();
</script>

<style scoped>
.el-table {
    margin-bottom: 50px !important;
}
</style>
