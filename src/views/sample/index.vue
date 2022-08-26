<template>
    <span>样本管理</span>
    <el-divider></el-divider>
    <div class="app-container">
        <div>
            <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
                <el-form-item label="医院名称" prop="hospital_id">
                    <el-select v-model="queryParams.hospital_id" placeholder="医院名称" clearable class="query_form_item"
                        @change="getInstrumentsOption(1)">
                        <el-option v-for="dict in hospitalsOption" :key="dict.key" :label="dict.label"
                            :value="dict.key" />
                    </el-select>
                </el-form-item>

                <el-form-item label="仪器名称" prop="instrument_id">
                    <el-select v-model="queryParams.instrument_id" placeholder="仪器名称" clearable class="query_form_item">
                        <el-option v-for="dict in instrumentsOption" :key="dict.key"
                            :label="dict.label + '-' + dict.code" :value="dict.key" />
                    </el-select>
                </el-form-item>

                <el-form-item label="条码编号" prop="sample_code">
                    <el-input class="query_form_item" v-model="queryParams.sample_code" placeholder="条码编号" clearable />
                </el-form-item>

                <el-form-item label="样本类型" prop="sample_type">
                    <el-select v-model="queryParams.sample_type" placeholder="仪器名称" clearable class="query_form_item">
                        <el-option v-for="dict in mc_sample_type" :key="dict.value" :label="dict.label"
                            :value="dict.label" />
                    </el-select>
                </el-form-item>

                <el-form-item label="项目组别" prop="test_group">
                    <el-select v-model="queryParams.test_group" placeholder="项目组别" clearable class="query_form_item">
                        <el-option v-for="dict in mc_test_group" :key="dict.value" :label="dict.label"
                            :value="dict.label" :disabled="dict.status == '0'" />
                    </el-select>
                </el-form-item>

                <el-form-item label="试剂批次" prop="regent_lot">
                    <el-input class="query_form_item" v-model="queryParams.regent_lot" placeholder="试剂批次" clearable />
                </el-form-item>

                <el-form-item label="样本状态" prop="status">
                    <el-select class="query_form_item" v-model="queryParams.status" placeholder="样本状态" clearable>
                        <el-option v-for="dict in mc_sample_status" :key="dict.value" :label="dict.label"
                            :value="dict.label" />
                    </el-select>
                </el-form-item>

                <el-form-item label="开始时间" prop="begin_time">
                    <el-date-picker class="query_form_item" clearable v-model="queryParams.begin_time" type="date"
                        value-format="YYYY-MM-DD" placeholder="开始时间">
                    </el-date-picker>
                </el-form-item>

                <el-form-item label="结束时间" prop="end_time">
                    <el-date-picker class="query_form_item" clearable v-model="queryParams.end_time" type="date"
                        value-format="YYYY-MM-DD" placeholder="结束时间">
                    </el-date-picker>
                </el-form-item>

                <el-form-item label="无效结果" prop="has_invalid_result">
                    <el-select class="query_form_item" v-model="queryParams.has_invalid_result" placeholder="无效结果"
                        clearable>
                        <el-option v-for="dict in has_invalid_result" :key="dict.key" :label="dict.key"
                            :value="dict.value" />
                    </el-select>
                </el-form-item>
                <el-form-item label="项目一致" prop="is_abnormal">
                    <el-select class="query_form_item" v-model="queryParams.is_abnormal" placeholder="项目一致性" clearable>
                        <el-option v-for="dict in is_abnormal" :key="dict.key" :label="dict.key" :value="dict.value" />
                    </el-select>
                </el-form-item>

                <el-form-item label="结果导入">
                    <el-checkbox class="query_form_item" v-model="queryParams.has_import_result">未导入结果样本
                    </el-checkbox>
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" :icon="Search" @click="getList">搜索</el-button>
                    <el-button :icon="Refresh" @click="resetQuery">重置</el-button>
                </el-form-item>
            </el-form>

            <el-row :gutter="10" class="mb8" style="height: 35px">
                <el-col :span="1.5">
                    <el-button type="primary" plain :icon="Plus" @click="handleSampleAdd">新增样本</el-button>
                </el-col>
                <el-col :span="1.5">
                    <el-button type="success" plain :icon="Rank" @click="handleSampleResultAdd">新增结果</el-button>
                </el-col>
                <el-col :span="1.5">
                    <el-button type="danger" plain :icon="Delete" :disabled="multiple" @click="handleDelete">彻底删除
                    </el-button>
                </el-col>
                <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
            </el-row>

            <el-table v-loading="loading" border :data="dataList" @selection-change="handleSelectionChange">
                <el-table-column type="selection" width="55" align="center" />
                <el-table-column label="#" align="center" width="50" type="index" />
                <el-table-column label="医院名称" align="center" prop="hospital_name" :show-overflow-tooltip="true"
                    width="180" />
                <el-table-column label="条码" align="center" prop="sample_code" :show-overflow-tooltip="true" width="150">
                    <template #default="scope">
                        <el-link type="success" @click="goToData(scope.row.id)">{{ scope.row.sample_code }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column label="类型" align="center" width="100" prop="sample_type" />

                <el-table-column label="项目组" align="center" width="100" prop="test_group" />
                <el-table-column label="状态" align="center" width="80" prop="status" :show-overflow-tooltip="true" />
                <el-table-column label="试剂批次" align="center" width="120" prop="regent_lot" />
                <el-table-column label="备注" align="center" prop="remark" :show-overflow-tooltip="true">
                </el-table-column>
                <el-table-column label="样本描述" align="center" prop="desc" :show-overflow-tooltip="true" />
                <el-table-column label="测试时间" align="center" prop="test_time" width="160">
                    <template #default="scope">
                        <span>{{ parseTime(scope.row.test_time) }}</span>
                    </template>
                </el-table-column>
                <el-table-column label="上传时间" align="center" prop="created_at" width="160">
                    <template #default="scope">
                        <span>{{ parseTime(scope.row.created_at) }}</span>
                    </template>
                </el-table-column>
            </el-table>

            <pagination v-show="total > 0" :total="total" v-model:page="queryParams.page_num"
                v-model:limit="queryParams.page_size" @pagination="getList" />
        </div>

    </div>

    <!-- 添加或修改岗位对话框 -->
    <el-dialog :title="title" v-model="open" width="400px" append-to-body>
        <el-form ref="formRef" :model="form" :rules="rules" label-width="100px">
            <el-progress v-if="!isUploadSample" :text-inside="true" :stroke-width="24" :percentage="dirProgressValue"
                :color="progressColors" :format="progressValueFormat" style="margin-bottom: 20px"></el-progress>
            <el-form-item label="医院名称" prop="name">
                <el-input disabled class="form_item" v-model="hospitalsMap[queryParams.hospital_id]"
                    placeholder="医院名称" />
            </el-form-item>
            <el-form-item label="仪器名称" prop="Ins_name">
                <el-input disabled class="form_item" v-model="instrumentsMap[queryParams.instrument_id].name"
                    placeholder="仪器名称" />
            </el-form-item>
            <el-form-item label="仪器编码" prop="Ins_code">
                <el-input disabled class="form_item" v-model="instrumentsMap[queryParams.instrument_id].code"
                    placeholder="仪器编码" />
            </el-form-item>
            <el-form-item label="仪器SN" prop="Ins_sn">
                <el-input disabled class="form_item" v-model="instrumentsMap[queryParams.instrument_id].sn"
                    placeholder="仪器SN" />
            </el-form-item>
            <el-form-item v-if="!isUploadSample" label="样本Id" prop="sample_id">
                <el-input disabled class="form_item" v-model="ids[0]" placeholder="样本Id" />
            </el-form-item>
            <!--        区分处理不同的excel-->
            <el-form-item v-if="isUploadSample">
                <template #label>
                    <span>
                        <el-tooltip placement="top">
                            <template #content>
                                <div>
                                    选择你上传的Excel表格数据类型，
                                    <br />Excel为软件安装目录下的Excel文件 <br />传统:早期导出数据类型，一行为一个检测结果
                                    <br />新式:更新后导出excel结果，一个样本为一行，所有结果在一行内
                                </div>
                            </template>
                            <el-icon>
                                <InfoFilled />
                            </el-icon>
                        </el-tooltip>
                        数据类型
                    </span>
                </template>
                <el-radio-group v-model="soft_excel_type">
                    <el-radio-button label="T">传统</el-radio-button>
                    <el-radio-button label="N" disabled>新式</el-radio-button>
                </el-radio-group>
            </el-form-item>
            <!-- 上传组件 -->
            <el-form-item v-if="isUploadSample" label="Excel上传">
                <el-upload multiple ref="uploadSoft" accept=".xls, .xlsx" action="" :auto-upload="false"
                    :show-file-list="false" :on-change="handleChangeSoftExcel">
                    <el-button type="primary">选取 导出Excel 文件上传 </el-button>
                </el-upload>
            </el-form-item>
            <!--        区分处理不同的excel-->
            <el-form-item v-if="!isUploadSample">
                <template #label>
                    <span>
                        <el-tooltip placement="top">
                            <template #content>
                                <div>
                                    选择你上传的Excel表格类型，
                                    <br />Excel为软件安装目录下的Excel文件 <br />传统:安装目录下
                                    analyze 文件夹下的Excel文件 <br />加密:安装目录下 temp
                                    文件夹下的加密Excel文件
                                    <br />需要专门解密后，然后使用专用工具导出文件
                                    <br />其格式为:文件夹日期@文件名.xls
                                    <br />如:2021-10-29@073633488_1.xls
                                </div>
                            </template>
                            <el-icon>
                                <InfoFilled />
                            </el-icon>
                        </el-tooltip>
                        Excel类型
                    </span>
                </template>
                <el-radio-group v-model="dir_excel_type">
                    <el-radio-button label="N">传统</el-radio-button>
                    <el-radio-button label="E">加密</el-radio-button>
                </el-radio-group>
            </el-form-item>
            <el-form-item v-if="!isUploadSample">
                <template #label>
                    <span>
                        <el-tooltip placement="top">
                            <template #content>
                                <div>
                                    是否强制更新
                                    <br />默认非强制更新 <br />非强制更新:已上传数据不更新
                                    <br />强制更新:已上传数据强制更新
                                </div>
                            </template>
                            <el-icon>
                                <InfoFilled />
                            </el-icon>
                        </el-tooltip>
                        强制更新
                    </span>
                </template>
                <el-checkbox class="form_item" v-model="force_update">开启强制更新</el-checkbox>
            </el-form-item>
            <el-form-item v-if="!isUploadSample">
                <template #label>
                    <span>
                        <el-tooltip placement="top">
                            <template #content>
                                <div>
                                    Excel上传
                                    <br />可单个上传 <br />也可批量上传
                                </div>
                            </template>
                            <el-icon>
                                <InfoFilled />
                            </el-icon>
                        </el-tooltip>
                        Excel上传
                    </span>
                </template>
                <el-upload multiple ref="uploadDir" accept=".xls, .xlsx" action="" :auto-upload="false"
                    :show-file-list="false" :on-change="handleChangeDirExcel">
                    <el-button type="success">选取 analyze Excel 文件上传</el-button>
                </el-upload>
            </el-form-item>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="cancel">取 消</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="McSample">
import { getCurrentInstance, ref, toRefs, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { Plus, Rank, Delete, Search, Refresh, InfoFilled } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox, ElNotification } from "element-plus"

import { readExcel } from '@/utils/excelUtils';


const { proxy } = getCurrentInstance();

const { mc_sample_type, mc_sample_status, mc_test_group } = proxy.useDict(
    'mc_sample_type',
    'mc_sample_status',
    'mc_test_group'
);

const has_invalid_result = ref([
    { key: '包含无效样本', value: '1' },
    { key: '不包无效样本', value: '0' },
]);
const is_abnormal = ref([
    { key: '测试数一致样本', value: '0' },
    { key: '测试不一致样本', value: '1' },
]);

const dataList = ref([]);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const isUploadSample = ref(true);
const force_update = ref(false);
//
//
const uploadSoft = ref();
const uploadDir = ref();

const dir_excel_type = ref('N');
const soft_excel_type = ref('T');
//
const ids = ref([]);
const names = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref('');
//
const hospitalsOption = ref([]);
const hospital_ids = ref([]);
const hospitalsMap = ref({});
const instrumentsOption = ref([]);
const instrumentsMap = ref({});
//
const dir_excel_count = ref(0);
const dirProgressValue = ref(0.0);
const progressColors = ref([
    { color: '##a5d8ff', percentage: 10 },
    { color: '##74c0fc', percentage: 20 },
    { color: '##4dabf7', percentage: 30 },
    { color: '##339af0', percentage: 40 },
    { color: '##228be6', percentage: 50 },
    { color: '##1c7ed6', percentage: 60 },
    { color: '##1971c2', percentage: 70 },
    { color: '##1864ab', percentage: 80 },
    { color: '#364fc7', percentage: 90 },
    { color: '##364fc7', percentage: 100 },
]);

const data = reactive({
    form: {},
    queryParams: {
        page_num: 1,
        page_size: 10,
        has_import_result: false,
        is_delete: false,
    },
    rules: {
        name: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
        province: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
        city_id: [{ required: true, message: '名称不能为空', trigger: 'blur' }],
        sort: [{ required: true, message: '排序不能为空', trigger: 'blur' }],
    },
});

const { queryParams, form, rules } = toRefs(data);

/** 查询岗位列表 */
async function getList() {
    loading.value = true;
    if (
        queryParams.value.end_time &&
        queryParams.value.begin_time &&
        queryParams.value.end_time < queryParams.value.begin_time
    ) {
        ElMessage.error('结束时间不能小于开始时间');
        loading.value = false;
        return;
    }
    const tQueryParams = { ...queryParams.value };
    if (queryParams.value.hospital_id === undefined || queryParams.value.hospital_id === '') {
        tQueryParams.hospital_ids = hospital_ids.value.join(',');
    }

    const qp = {
        pageParams: {
            page_num: tQueryParams.page_num,
            page_size: tQueryParams.page_size,
        },
        req: {
            ...tQueryParams
        }
    }


    const [res, msg] = await invoke("get_sample_list", { ...qp })
    if (msg !== null) {
        ElMessage.error(msg)
        loading.value = false;
        return
    }
    dataList.value = res.list;
    total.value = res.total;

    loading.value = false;

}
// 获取选项值
async function getHospitalsOption() {
    loading.value = true;
    const qp = {
        pageParams: {
            page_num: 1,
            page_size: Number.MAX_SAFE_INTEGER
        },
        req: {
        }
    }
    const [res, msg] = await invoke("get_hospital_list", { ...qp })
    if (msg == !null) {
        ElMessage.error(msg)
        loading.value = false;
        return
    }

    const data = res.list;


    const _hospital_ids = [];
    data.forEach((element) => {
        hospitalsOption.value.push({
            key: element.id,
            label: element.name,
        });
        _hospital_ids.push(element.id);
        hospitalsMap.value[element.id] = element.name;
    });
    hospital_ids.value = _hospital_ids;
    loading.value = false;
}
async function getInstrumentsOption(v) {
    loading.value = true;
    instrumentsOption.value = [];

    const qp = {
        pageParams: {
            page_num: 1,
            page_size: Number.MAX_SAFE_INTEGER
        },
        req: {
            hospital_id: queryParams.value.hospital_id,
        }
    }
    const [res, msg] = await invoke("get_instrument_list", { ...qp })
    if (msg !== null) {
        ElMessage.error("获取医院仪器数据错误:", msg)
        loading.value = false;
    }
    console.log(res)
    res.list.forEach((element) => {
        instrumentsOption.value.push({
            key: element.id,
            label: element.name,
            code: element.code,
        });
        instrumentsMap.value[element.id] = {
            name: element.name,
            code: element.code,
            sn: element.sn,
        };
    });
    if (v == 1) {
        queryParams.value.instrument_id = undefined;
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
/** 新增样本按钮操作 */
function handleSampleAdd() {
    isUploadSample.value = true;
    dir_excel_count.value = 0;
    dirProgressValue.value = 0.0;
    if (queryParams.value.instrument_id == undefined) {
        ElMessage.warning('请先选择医院和仪器');
        return;
    }
    open.value = true;
    title.value = '新增样本';
}
function handleSampleResultAdd() {
    isUploadSample.value = false;
    dir_excel_count.value = 0;
    dirProgressValue.value = 0.0;
    if (queryParams.value.instrument_id == undefined) {
        ElMessage.error('请先选择医院和仪器');
        return;
    }
    open.value = true;
    title.value = '新增DIR结果';
}

async function handleChangeSoftExcel(file, fileList) {
    if (soft_excel_type.value === 'T') {
        return await handleChangeSoftExcel_T(file, fileList)
    } else {
        return await handleChangeSoftExcel_N(file, fileList)
    }
}

// 新式导出数据处理
async function handleChangeSoftExcel_N(file, fileList) {
    const _file = file.raw;
    const _excelData = await readExcel(_file, false, 1);
    if (_excelData[0][0] !== '测试时间') {
        proxy.$refs.uploadSoft.clearFiles();
        ElMessage.error(
            '请确认导入正确的excel表格，选择正确的excel数据类型'
        );
        return;
    }
    const excel_data = {
        base_info: {
            hospital_id: queryParams.value.hospital_id,
            hospital_name: hospitalsMap.value[queryParams.value.hospital_id],
            instrument_id: queryParams.value.instrument_id,
            instrument_code:
                instrumentsMap.value[queryParams.value.instrument_id].code,
            instrument_sn: instrumentsMap.value[queryParams.value.instrument_id].sn,
        },
        excel_data: _excelData,
    };

    await invoke("add_sample_n", { req: excel_data });
    ElNotification.success('数据上传成功，请等待系统处理');
    if (file.name == fileList[fileList.length - 1].name) {
        proxy.$refs.uploadSoft.clearFiles();
        open.value = false;
        await getList();
    }

}

// 传统导出数据处理
async function handleChangeSoftExcel_T(file, fileList) {
    const _file = file.raw;
    let header = [
        'sample_code',
        'sample_type',
        'test_group',
        'status',
        'test_time',
        'regent_lot',
        'remark',
        'test_id',
        'test_name',
        'result_count',
        'result_signal',
        'result_ai',
        'result_index',
    ];
    const _excelData = await readExcel(_file, false, header);
    if (
        _excelData[0].sample_code !== '样本条码' &&
        _excelData[0].test_id !== '磁条码编码'
    ) {
        proxy.$refs.uploadSoft.clearFiles();
        ElMessage.error(
            '请确认导入正确的excel表格->管理员账号下软件历史结果导出的excel'
        );
        return;
    }
    const excel_data = {
        base_info: {
            hospital_id: queryParams.value.hospital_id,
            hospital_name: hospitalsMap.value[queryParams.value.hospital_id],
            instrument_id: queryParams.value.instrument_id,
            instrument_code:
                instrumentsMap.value[queryParams.value.instrument_id].code,
            instrument_sn: instrumentsMap.value[queryParams.value.instrument_id].sn,
        },
        excel_data: _excelData.slice(1),
    };
    //
    await invoke("add_sample", { req: excel_data });

    ElNotification.success('数据上传成功，请等待系统处理');
    if (file.name == fileList[fileList.length - 1].name) {
        proxy.$refs.uploadSoft.clearFiles();
        open.value = false;
        await getList();
    }
}

async function handleChangeDirExcel(file, fileList) {
    console.log(fileList)
    const _file = file.raw;
    const _excelData = await readExcel(_file, false, 1);
    if (_excelData[0][0] !== '编号') {
        proxy.$refs.uploadDir.clearFiles();
        ElMessage.error(
            '请确认导入正确的excel表格->软件安装目录下[analyze]目录下的excel'
        );
        return;
    }

    // const _sampleCode = file.name
    //   .toString()
    //   .split('_')
    //   .slice(1)
    //   .join('_')
    //   .split('.');
    // _sampleCode.pop();
    // const _testDateTime = new Date(
    //   +_file.lastModifiedDate + 8 * 3600 * 1000
    // ).toISOString();
    // const t_time = proxy.parseTime(_testDateTime, '{y}-{m}-{d} {h}:{i}:{s}');

    const { sample_code, test_time } = get_sample_test_time(file);


    const _temExcelData = await splitDirExcelData(_excelData);
    console.log(_temExcelData)
    // 在这个位置 可能比较好

    if (_temExcelData.length <= 3) {
        dir_excel_count.value += 1;
        if (file.name == fileList[fileList.length - 1].name) {
            proxy.$refs.uploadDir.clearFiles();
            open.value = false;
            await getList();
        }
        return;
    }
    const excelData = {
        base_info: {
            hospital_id: queryParams.value.hospital_id,
            hospital_name: hospitalsMap.value[queryParams.value.hospital_id],
            instrument_id: queryParams.value.instrument_id,
            instrument_code:
                instrumentsMap.value[queryParams.value.instrument_id].code,
            instrument_sn: instrumentsMap.value[queryParams.value.instrument_id].sn,
            sample_code: sample_code,
            test_time: test_time,
            sample_id: ids.value.length > 0 ? ids.value[0] : undefined,
            force_update: force_update.value,
        },

        excel_data: _temExcelData,
    };
    dir_excel_count.value += 1;
    dirProgressValue.value = parseFloat(
        ((100 * dir_excel_count.value) / fileList.length).toFixed(3)
    );
    await invoke("add_sample_result", { req: excelData });
    if (file.name == fileList[fileList.length - 1].name) {
        proxy.$refs.uploadDir.clearFiles();
        open.value = false;
        await getList();
    }
}

// 获取样本检测时间
const get_sample_test_time = (file) => {
    let t_time = '';
    let sample_code = '';
    if (dir_excel_type.value === 'N') {
        // 测试时间
        const _file_time = file.raw.lastModifiedDate || file.raw.lastModified
        const _testDateTime = new Date(
            +_file_time + 8 * 3600 * 1000
        ).toISOString();

        t_time = proxy.parseTime(_testDateTime, '{y}-{m}-{d} {h}:{i}:{s}');
        // 样本条码
        sample_code = file.name.toString().split('_').slice(1).join('_').split('.');
        sample_code.pop();
    } else {
        if (file.name.length < 17) {
            ElMessage.error(
                '请确认导入正确的excel表格->解密后且用工具导出的的excel'
            );
            return;
        }
        // 测试时间
        const t_data_time_str = file.name.slice(0, 17);
        const t_data_time_list = t_data_time_str.split('@');
        if (
            t_data_time_list.length !== 2 ||
            t_data_time_list[0].length !== 10 ||
            t_data_time_list[1].length !== 6
        ) {
            ElMessage.error(
                '请确认导入正确的excel表格->解密后且用工具导出的的excel'
            );
            return;
        }
        t_time =
            t_data_time_list[0] +
            ' ' +
            t_data_time_list[1].slice(0, 2) +
            ':' +
            t_data_time_list[1].slice(2, 4) +
            ':' +
            t_data_time_list[1].slice(4, 6);
        // 样本条码
        sample_code = file.name.toString().split('_').slice(1).join('_').split('.');
        sample_code.pop();
    }
    return {
        sample_code: sample_code.join('.'),
        test_time: t_time,
    };
};

function splitDirExcelData(dirExcelData) {
    let index = 0;
    for (let i = 0; i < dirExcelData.length; i++) {
        if (dirExcelData[i][0] === '合计数量') {
            index = i;
            break;
        }
    }
    const result = dirExcelData.slice(index);
    return result;
}

/** 删除按钮操作 */
function handleDelete() {
    ElMessageBox
        .confirm('是否确认永久删除该"  ' + ids.value.length + '  "项数据？')
        .then(async () => {
            const [_, msg] = await invoke("delete_sample", { ids: ids.value });
            if (msg !== null) {
                ElMessage.error(msg)
                return
            }
            getList();
            ElMessage.success('删除成功');

        })
        .catch(() => { });
}

const progressValueFormat = (v) => {
    return '文件处理进度' + v + '%';
};

//  数据页面跳转
const goToData = (id) => {
  proxy.$router.push({path:"/sample_result",query: {id}});
}

const init = async () => {
    await getHospitalsOption();
    await getList();
};

init();
</script>

<style lang="scss" scoped>
.el-link {
  margin-right: 8px;
}
</style>
