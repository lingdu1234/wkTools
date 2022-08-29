<template>
  <div class="app-container">
    <div>
      <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
        <el-form-item label="医院名称" prop="hospital_id">
          <el-select v-model="queryParams.hospital_id" placeholder="医院名称" clearable class="query_form_item"
            @change="getInstrumentsOption(1)">
            <el-option v-for="dict in hospitalsOption" :key="dict.key" :label="dict.label" :value="dict.key" />
          </el-select>
        </el-form-item>

        <el-form-item label="仪器名称" prop="instrument_id">
          <el-select v-model="queryParams.instrument_id" placeholder="仪器名称" clearable class="query_form_item">
            <el-option v-for="dict in instrumentsOption" :key="dict.key" :label="dict.label + '-' + dict.code"
              :value="dict.key" />
          </el-select>
        </el-form-item>

        <el-form-item label="条码编号" prop="sample_code">
          <el-input class="query_form_item" v-model="queryParams.sample_code" placeholder="条码编号" clearable />
        </el-form-item>

        <el-form-item label="样本类型" prop="sample_type">
          <el-select v-model="queryParams.sample_type" placeholder="仪器名称" clearable class="query_form_item">
            <el-option v-for="dict in mc_sample_type" :key="dict.value" :label="dict.label" :value="dict.label" />
          </el-select>
        </el-form-item>

        <el-form-item label="项目组别" prop="test_group">
          <el-select v-model="queryParams.test_group" placeholder="项目组别" clearable class="query_form_item">
            <el-option v-for="dict in mc_test_group" :key="dict.value" :label="dict.label" :value="dict.label"
              :disabled="dict.status === '0'" />
          </el-select>
        </el-form-item>

        <el-form-item label="试剂批次" prop="regent_lot">
          <el-input class="query_form_item" v-model="queryParams.regent_lot" placeholder="试剂批次" clearable />
        </el-form-item>

        <el-form-item label="样本状态" prop="status">
          <el-select class="query_form_item" v-model="queryParams.status" placeholder="样本状态" clearable>
            <el-option v-for="dict in mc_sample_status" :key="dict.value" :label="dict.label" :value="dict.label" />
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
          <el-select class="query_form_item" v-model="queryParams.has_invalid_result" placeholder="无效结果" clearable>
            <el-option v-for="dict in has_invalid_result" :key="dict.key" :label="dict.key" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-form-item label="项目一致" prop="is_abnormal">
          <el-select class="query_form_item" v-model="queryParams.is_abnormal" placeholder="项目一致性" clearable>
            <el-option v-for="dict in is_abnormal" :key="dict.key" :label="dict.key" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" icon="Search" @click="getList">搜索</el-button>
          <el-button icon="Refresh" @click="resetQuery">重置</el-button>
        </el-form-item>
      </el-form>
      <el-divider content-position="left">数据导出操作区域</el-divider>
      <el-row :gutter="10" justify="space-around">
        <el-col :xs="24" :sm="24" :md="24" :lg="12" :xl="12" align="center">
          <table class="pure-table pure-table-bordered">
            <thead>
              <tr>
                <th>#</th>
                <th>数据类型</th>
                <th>统计选项</th>
                <th>拉取数据</th>
                <th>导出数据</th>
              </tr>
            </thead>

            <tbody>
              <tr>
                <td>A</td>
                <td>原始数据</td>
                <td></td>
                <td>
                  <el-button size="small" type="warning" @click="getAllOriginData">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="warning" style="width: 60px"
                    :disabled="exportOutData.OriginData.length === 0" @click="
                      exportDataExcelOrCsv(
                        'Excel',
                        exportOutData.OriginData,
                        '原始数据'
                      )
                    ">Excel
                  </el-button>
                  <el-button size="small" type="warning" style="width: 60px"
                    :disabled="exportOutData.OriginData.length === 0" @click="
                      exportDataExcelOrCsv(
                        'CSV',
                        exportOutData.OriginData,
                        '原始数据'
                      )
                    ">CSV
                  </el-button>
                </td>
              </tr>
              <tr class="pure-table-odd">
                <td>B</td>
                <td>样本统计</td>
                <td></td>
                <td>
                  <el-button size="small" type="info" @click="getTestCount">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="info" style="width: 60px"
                    :disabled="exportOutData.SampleCountData_A.length === 0" @click="exportSampleCountData('A')">格式A
                  </el-button>
                  <el-button size="small" type="info" style="width: 60px"
                    :disabled="exportOutData.SampleCountData_B.length === 0" @click="exportSampleCountData('B')">格式B
                  </el-button>
                </td>
              </tr>
              <!--           无效磁片统计-->
              <tr>
                <td>C</td>
                <td>无效统计</td>
                <td>
                  <el-button size="small" type="primary" @click="
                    () => {
                      tongJiDialogTitle = '无效统计设置';
                      tongJiSettingDialog = true;
                    }
                  ">选项设置
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" @click="getInvalidCount()">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" style="width: 60px"
                    :disabled="exportOutData.invalidResult.length === 0" @click="
                      exportDataExcelOrCsv(
                        'Excel',
                        exportOutData.invalidResult,
                        '无效样本统计'
                      )
                    ">Excel
                  </el-button>
                </td>
              </tr>
              <!-- 无效样本 -->
              <tr class="pure-table-odd">
                <td>D</td>
                <td>无效样本</td>
                <td></td>
                <td>
                  <el-button size="small" type="primary" plain @click="getInvalidSample()">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" plain style="width: 60px"
                    :disabled="exportOutData.invalidSample.length === 0" @click="
                      exportDataExcelOrCsv(
                        'Excel',
                        exportOutData.invalidSample,
                        '无效样本'
                      )
                    ">Excel
                  </el-button>
                </td>
              </tr>
            </tbody>
          </table>
        </el-col>
        <!-- 右边表格 -->
        <el-col :xs="24" :sm="24" :md="24" :lg="12" :xl="12" align="center">
          <table class="pure-table pure-table-bordered">
            <thead>
              <tr>
                <th>#</th>
                <th>数据类型</th>
                <th>统计选项</th>
                <th>拉取数据</th>
                <th>导出数据</th>
              </tr>
            </thead>

            <tbody>
              <!--          结果数据New-->
              <tr>
                <td>A</td>
                <td>结果数据</td>
                <td>
                  <el-button size="small" type="warning" @click="OpenResultSettingDialog">选项设置
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="warning" @click="getAllSampleData">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="warning" style="width: 60px"
                    :disabled="exportOutData.allSample.length === 0" @click="
                      exportDataExcelOrCsv(
                        'Excel',
                        exportOutData.allSample,
                        '样本结果'
                      )
                    ">Excel
                  </el-button>
                  <el-button size="small" type="warning" style="width: 60px"
                    :disabled="exportOutData.allSample.length === 0" @click="
                      exportDataExcelOrCsv(
                        'CSV',
                        exportOutData.allSample,
                        '样本结果'
                      )
                    ">CSV
                  </el-button>
                </td>
              </tr>

              <tr class="pure-table-odd">
                <td>B</td>
                <td>阳性率</td>
                <td>
                  <el-button size="small" type="primary" @click="openPositiveRateDialog">选项设置
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" @click="getPositiveRateData">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" style="width: 60px"
                    :disabled="exportOutData.positiveRateData.length === 0" @click="
                      exportDataExcelOrCsv(
                        'Excel',
                        exportOutData.positiveRateData,
                        '阳性率统计'
                      )
                    ">Excel
                  </el-button>
                </td>
              </tr>

              <tr>
                <td>C</td>
                <td>磁片统计</td>
                <td>
                  <el-button size="small" type="primary" @click="openMagnetSettingDialog">选项设置
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" @click="getCiPianData">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" style="width: 60px"
                    :disabled="exportOutData.CiPianCountData.length === 0" @click="
                      exportDataExcelOrCsv(
                        'Excel',
                        exportOutData.CiPianCountData,
                        '磁片统计'
                      )
                    ">Excel
                  </el-button>
                </td>
              </tr>
              <tr class="pure-table-odd">
                <td>D</td>
                <td>本底范围</td>
                <td>
                  <el-button size="small" type="primary" @click="openBenDiRangeSettingDialog">选项设置
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" @click="getBenDiRange">获取数据
                  </el-button>
                </td>
                <td>
                  <el-button size="small" type="primary" style="width: 60px"
                    :disabled="exportOutData.BenDiRangeData.length === 0" @click="
                      exportDataExcelOrCsv(
                        'Excel',
                        exportOutData.BenDiRangeData,
                        '本底范围'
                      )
                    ">Excel
                  </el-button>
                </td>
              </tr>
            </tbody>
          </table>
        </el-col>
      </el-row>

      <el-divider content-position="left">数据导出操作区域</el-divider>

      <el-table v-loading="loading" border :data="dataList">
        <el-table-column label="#" align="center" width="50" type="index" />

        <el-table-column label="医院名称" align="center" prop="hospital_name" :show-overflow-tooltip="true" width="180" />
        <el-table-column label="条码" align="center" prop="sample_code" :show-overflow-tooltip="true" width="150">
          <template #default="scope">
            <el-link type="success" @click="goToData(scope.row.id)">{{  scope.row.sample_code  }}
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
            <span>{{  parseTime(scope.row.test_time)  }}</span>
          </template>
        </el-table-column>
        <el-table-column label="上传时间" align="center" prop="created_at" width="160">
          <template #default="scope">
            <span>{{  parseTime(scope.row.created_at)  }}</span>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.page_num"
        v-model:limit="queryParams.page_size" @pagination="getList" />
    </div>
  </div>
  <!-- 对话框 -->
  <!-- 阳性率统计设置对话框 -->
  <el-dialog :title="tongJiDialogTitle" v-model="tongJiSettingDialog" :before-close="beforeTongJiDialogClose"
    width="500px" append-to-body>
    <el-form label-width="80px">
      <!--         阳性率设置选项-->
      <el-form-item label="统计选项:" v-if="positiveRateOption.isShow">
        <el-radio-group v-model="positiveRateOption.optionSelected">
          <el-radio v-for="option in positiveRateOption.options" :key="option.label" :label="option.label">{{
             option.tagName 
            }}
          </el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="统计规则:" v-if="positiveRateOption.isShow">
        <!--          使用 ！== 注意类型-->
        <el-input :disabled="positiveRateOption.optionSelected !== '2'" v-model="positiveRateOption.optionValue"
          placeholder=""></el-input>
      </el-form-item>
      <!--        磁片数统计选项-->
      <el-form-item label="统计选项:" v-if="CiPianOptions.isShow">
        <el-radio-group v-model="CiPianOptions.optionSelected">
          <el-radio v-for="option in CiPianOptions.options" :key="option.label" :label="option.label">{{  option.tagName 
            }}
          </el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="统计规则:" v-if="CiPianOptions.isShow">
        <!--          使用 ！== 注意类型-->
        <el-input :disabled="CiPianOptions.optionSelected !== '2'" v-model="CiPianOptions.optionValue" placeholder="">
        </el-input>
      </el-form-item>
      <!--        本底统计选项-->
      <el-form-item label="统计选项:" v-if="BendiOptions.isShow">
        <el-radio-group v-model="BendiOptions.optionSelected">
          <el-radio v-for="option in BendiOptions.options" :key="option.label" :label="option.label">{{  option.tagName 
            }}
          </el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="统计规则:" v-if="BendiOptions.isShow">
        <!--          使用 ！== 注意类型-->
        <el-input :disabled="BendiOptions.optionSelected !== '2'" v-model="BendiOptions.optionValue" placeholder="">
        </el-input>
      </el-form-item>
      <el-form-item label="统计分组">
        <table class="pure-table pure-table-bordered" style="width: 370px">
          <tbody>
            <tr>
              <td>医院</td>
              <td>
                <el-switch v-model="tongJiQueryParams.byHospital"></el-switch>
              </td>
              <td>仪器</td>
              <td>
                <el-switch v-model="tongJiQueryParams.byInstrument"></el-switch>
              </td>
            </tr>

            <tr class="pure-table-odd">
              <td>项目</td>
              <td>
                <el-switch v-model="tongJiQueryParams.byTestGroup"></el-switch>
              </td>
              <td>批号</td>
              <td>
                <el-switch v-model="tongJiQueryParams.byRegentLot"></el-switch>
              </td>
            </tr>
            <tr>
              <td>月份</td>
              <td>
                <el-switch v-model="tongJiQueryParams.byMonth"></el-switch>
              </td>
              <td>测试</td>
              <td>
                <el-switch v-model="tongJiQueryParams.byTestName"></el-switch>
              </td>
            </tr>
          </tbody>
        </table>
      </el-form-item>
      <el-form-item label="说明">
        <el-tag type="warning">项目 如ANA-17等,测试 如dsDNA 等</el-tag>
      </el-form-item>
    </el-form>
    <div slot="footer" class="dialog-footer"></div>
  </el-dialog>
  <!-- 结果数据统计对话 -->
  <el-dialog :title="resultSettingTitle" v-model="openResultSettingDialog" width="500px" append-to-body>
    <el-form label-width="80px">
      <el-form-item label="提示">
        <el-tag type="danger">注意选择顺序,影响结果顺序,不确定全取消重选</el-tag>
      </el-form-item>
      <el-form-item label="选项值">
        <span>{{  resultOptionValue  }}</span>
      </el-form-item>
      <el-form-item label="统计选项">
        <el-checkbox-group v-model="resultOptionValue">
          <el-checkbox-button v-for="it in resultOption" :label="it.key" :key="it.key">{{  it.Tag  }}
          </el-checkbox-button>
        </el-checkbox-group>
      </el-form-item>
    </el-form>
    <div slot="footer" class="dialog-footer"></div>
  </el-dialog>
</template>

<script setup name="McStatistics">
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';
import { getCurrentInstance, ref, toRefs, reactive } from 'vue';
import { ElMessage, ElMessageBox, ElNotification } from 'element-plus';

// import {
//   exportSingleListData2excel,
//   exportSingleListData2csv,
//   exportSingleJsonData2excel,
// } from '@/utils/excelUtils';
import {
  ObjectList_to_2dList,
  ObjectList_to_childList,
  childList_to_listlist,
  transfor_sample_result,
} from '@/utils/dataConvert';

import Instrument from './sample_result.vue';

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
//
// 统计选项
const tongJiQueryParams = ref({
  byHospital: true,
  byInstrument: true,
  byTestGroup: true,
  byRegentLot: true,
  byTestName: true,
  byMonth: true,
});
const tongJiSettingDialog = ref(false); //对话框
const tongJiDialogTitle = ref(''); //标题
const positiveRateOption = ref({
  isShow: false,
  options: [
    {
      tagName: '常规',
      value: '0.5,0.8,0.9,1.0,1.1,1.2,2.0',
      label: '0',
    },
    { tagName: '默认', value: '0.8,1.0,1.2', label: '1' },
    {
      tagName: '自定义',
      value: '0.3,0.5,0.8,0.9,1.0,1.1,1.2,2.0,5.0',
      label: '2',
    },
  ],
  optionSelected: '1',
  optionValue: '0.8,1.0,1.2',
  inputDisabled: true,
});
const CiPianOptions = ref({
  isShow: false,
  options: [
    {
      tagName: '常规',
      value: '10,20,30,40,50',
      label: '0',
    },
    { tagName: '默认', value: '10,20,30', label: '1' },
    {
      tagName: '自定义',
      value: '10,20,30,40',
      label: '2',
    },
  ],
  optionSelected: '1',
  optionValue: '10,20,30',
  inputDisabled: true,
});
// 本底统计选项
const BendiOptions = ref({
  isShow: false,
  options: [
    {
      tagName: '常规',
      value: '120,140,160',
      label: '0',
    },
    { tagName: '默认', value: '110,120,130,140,150,160', label: '1' },
    {
      tagName: '自定义',
      value: '110,130,150',
      label: '2',
    },
  ],
  optionSelected: '1',
  optionValue: '110,120,130,140,150,160',
  inputDisabled: true,
});
// 样本数据选项
const resultSettingTitle = ref('');
const openResultSettingDialog = ref(false);
const resultOption = ref([
  { Tag: '磁片', key: 'N' },
  { Tag: '信号', key: 'S' },
  { Tag: '浓度', key: 'A' },
  { Tag: '定性', key: 'I' },
]);
const resultOptionValue = ref(['N', 'S', 'A']);
//
const exportOutData = ref({
  allSample: [], //全部样本数据
  OriginData: [], // 伪原始数据
  SampleCountData_A: [], // 样本统计数据
  SampleCountData_B: [], // 样本统计数据
  CiPianCountData: [], // 磁片统计数据
  BenDiRangeData: [], // 磁片统计数据
  invalidResult: [], //无效结果
  invalidSample: [], //无效样本
  positiveRateData: [], // 阳性率数据 格式A
});
//
const total = ref(0);
//
const hospitalsOption = ref([]);
const hospital_ids = ref([]);
const hospitalsMap = ref({});
const instrumentsOption = ref([]);
const instrumentsMap = ref({});
//

const data = reactive({
  form: {},
  queryParams: {
    page_num: 1,
    page_size: 10,
    has_import_result: false,
    is_delete: false,
  },
});

const { queryParams, form } = toRefs(data);

/** 查询样本列表 */
async function getList() {
  loading.value = true;
  if (
    queryParams.value.end_time &&
    queryParams.value.begin_time &&
    queryParams.value.end_time < queryParams.value.begin_time
  ) {
    ElMessage.warning('结束时间不能小于开始时间');
    loading.value = false;
    return;
  }
  const tQueryParams = { ...queryParams.value };
  if (
    queryParams.value.hospital_id === undefined ||
    queryParams.value.hospital_id === ''
  ) {
    tQueryParams.hospital_ids = hospital_ids.value.join(',');
  }
  const qp = {
    pageParams: {
      page_num: tQueryParams.page_num,
      page_size: tQueryParams.page_size,
    },
    req: {
      ...tQueryParams,
    },
  };
  const [res, msg] = await invoke('get_sample_list', { ...qp });
  if (msg !== null) {
    ElMessage.error(msg);
    loading.value = false;
    return;
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
      page_size: Number.MAX_SAFE_INTEGER,
    },
    req: {},
  };
  const [res, msg] = await invoke('get_hospital_list', { ...qp });
  if (msg !== null) {
    loading.value = false;
    ElMessage.error(msg);
    return;
  }
  const _hospital_ids = [];
  res.list.forEach((element) => {
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
      page_size: Number.MAX_SAFE_INTEGER,
    },
    req: {
      hospital_id: queryParams.value.hospital_id,
    },
  };
  const [res, msg] = await invoke('get_instrument_list', { ...qp });
  if (msg !== null) {
    loading.value = false;
    ElMessage.error(msg);
    return;
  }
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
  if (v === 1) {
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

function beforeTongJiDialogClose() {
  positiveRateOption.value.isShow = false;
  CiPianOptions.value.isShow = false;
  BendiOptions.value.isShow = false;
  tongJiSettingDialog.value = false;
}

/** 重置按钮操作 */
const resetQuery = async () => {
  proxy.resetForm('queryRef');
  queryParams.value.page_num = 1;
  await getList();
};

// 原始数据获取
async function getAllOriginData() {
  if (queryParams.value.hospital_id === undefined) {
    ElMessage.warning('请先选择医院后再操作');
    return;
  }

  const qp = {
    pageParams: {
      page_num: 1,
      page_size: Number.MAX_SAFE_INTEGER,
    },
    req: {
      hospital_id: queryParams.value.hospital_id,
    },
  };

  const [res, msg] = await invoke('get_origin_list', { ...qp });
  if (msg !== null) {
    ElMessage.error(msg);
    loading.value = false;
    return;
  }
  exportOutData.value.OriginData = [res.title_cn, ...res.list];
}


//  数据导出为excel
const exportDataExcelOrCsv = async (v, data, file_name) => {
  if (v === "Excel") {
    const file_path = await save();
    await invoke("write_array_data_to_excel", { fileName: file_path, excelData: data })
    ElMessage.success("数据导出完成")
  } else {
    const file_path = await save();
    await invoke("write_array_data_to_csv", { fileName: file_path, csvData: data })
    ElMessage.success("数据导出完成")
  }

};

// 样本统计结果
async function getTestCount() {
  if (queryParams.value.hospital_id === undefined) {
    ElMessage.warning('请先选择医院后再操作');
    return;
  }
  exportOutData.value.SampleCountData_A = [];
  exportOutData.value.SampleCountData_B = [];

  const qp = {
    opt: {
      time_option: 'M',
      group_opition: 'TG',
    },
    req: {
      ...queryParams.value,
    },
  };

  const [res, msg] = await await invoke('get_test_count', { ...qp });
  const commKey = ['医院', '仪器SN', '测试组'];
  const childKey = ['样本', '校准', '质控', '对照', '无效', '有效', '合计'];
  const childKey2 = ['合计', '样本'];
  const chilidList = ObjectList_to_childList(
    res.list,
    commKey,
    childKey,
    res.time_list,
    '月份'
  );
  const ddList_A = childList_to_listlist(
    chilidList,
    commKey,
    childKey2,
    res.time_list,
    'children'
  );
  const ddList_B = childList_to_listlist(
    chilidList,
    commKey,
    childKey,
    res.time_list,
    'children'
  );
  exportOutData.value.SampleCountData_A.push(...ddList_A, [], [], ...ddList_B);
  console.log('exportOutData.value.SampleCountData_A :>> ', exportOutData.value.SampleCountData_A);
  console.log('res.list :>> ', res.list);
  exportOutData.value.SampleCountData_B = res.list;
}

const exportSampleCountData = async (v) => {

  if (v === 'A') {
    const file_path = await save();
    await invoke("write_array_data_to_excel", { fileName: file_path, excelData: exportOutData.value.SampleCountData_A })
    ElMessage.success("数据导出完成")
  } else {
    let header = [
      '医院',
      '仪器SN',
      '测试组',
      '月份',
      '开始时间',
      '截止时间',
      '样本',
      '校准',
      '质控',
      '对照',
      '无效',
      '有效',
      '合计',
    ];
    console.log('B :>> ', exportOutData.value.SampleCountData_B);
    const file_path = await save();
    await invoke("write_array_map_data_to_excel", { fileName: file_path, header: header, excelData: exportOutData.value.SampleCountData_B })
    ElMessage.success("数据导出完成")
  }
};
// 无效结果统计
async function getInvalidCount() {
  exportOutData.value.invalidResult = [];
  if (queryParams.value.hospital_id == undefined) {
    ElMessage.warning('请先选择医院后再操作');
    return;
  }

  const qp = {
    opts: {
      by_hospital: tongJiQueryParams.value.byHospital,
      by_instrument: tongJiQueryParams.value.byInstrument,
      by_test_group: tongJiQueryParams.value.byTestGroup,
      by_regent_lot: tongJiQueryParams.value.byRegentLot,
      by_test_name: tongJiQueryParams.value.byTestName,
      time_option: tongJiQueryParams.value.byMonth ? 'M' : undefined,
    },
    req: {
      ...queryParams.value,
    },
  };

  const [data, msg] = await invoke('get_invalid_count', { ...qp });
  if (msg !== null) {
    ElMessage.error(msg);
    loading.value = false;
    return;
  }
  let key_cn = [
    '医院',
    '仪器SN',
    '月份',
    '开始时间',
    '截止时间',
    '测试组',
    '试剂批次',
    '项目',
    '校准',
    '质控',
    '对照',
    '样本',
    '合计样本',
    '合计测试',
    '样本无效率',
    '合计无效率',
  ];
  let key_en = [
    'hospital',
    'instrument',
    'month',
    'begin_time',
    'end_time',
    'test_group',
    'regent_lot',
    'test_name',
    'invalid_cal',
    'invalid_qc',
    'invalid_npc',
    'invalid_s',
    'sample_total',
    'all_total',
    'invalid_s_percent',
    'invalid_all_percent',
  ];
  exportOutData.value.invalidResult = ObjectList_to_2dList(
    data,
    key_en,
    key_cn
  );
}
//  获取无效样本
async function getInvalidSample() {
  if (queryParams.value.hospital_id == undefined) {
    ElMessage.error('请先选择医院后再操作');
    return;
  }
  let tQueryParams = { ...queryParams.value };
  tQueryParams.has_invalid_result = '1';

  const qp = {
    pageParams: {
      page_num: 1,
      page_size: Number.MAX_SAFE_INTEGER,
    },
    req: {
      ...tQueryParams,
    },
  };

  const [sample_res, msg1] = await invoke('get_sample_list', { ...qp });
  if (msg1 !== null) {
    ElMessage.error(msg1);
    loading.value = false;
    return;
  }
  const sample = sample_res.list;

  const [sample_r_res, msg2] = await invoke('get_sample_result_list', {
    ...qp,
  });
  if (msg2 !== null) {
    ElMessage.error(msg2);
    loading.value = false;
    return;
  }
  const result = sample_r_res.list;

  const key_cn_r = [
    '医院',
    '仪器',
    '条码',
    '类型',
    '项目组',
    '状态',
    '测试时间',
    '试剂批次',
    '总时长',
    '备注',
    '项目Id',
    '项目名称',
    '磁码计数',
    '信号',
    '浓度',
    '定性',
    '均值',
    '中位数',
    '最小值',
    '最大值',
    'CV',
  ];
  const key_cn_s = [
    '医院',
    '仪器',
    '条码',
    '类型',
    '项目组',
    '状态',
    '测试时间',
    '试剂批次',
    '备注',
    '描述',
  ];
  const key_en_s = [
    'hospital_name',
    'instrument_sn',
    'sample_code',
    'sample_type',
    'test_group',
    'sample_status',
    'test_time',
    'regent_lot',
    'remark',
    'desc',
  ];
  const key_en_r = [
    'hospital_name',
    'instrument_sn',
    'sample_code',
    'sample_type',
    'test_group',
    'sample_status',
    'test_time',
    'regent_lot',
    'total_time',
    'sample_remark',
    'test_id',
    'test_name',
    'result_count',
    'result_signal',
    'result_ai',
    'result_index',
    'result_avg',
    'result_med',
    'result_min',
    'result_max',
    'result_cv',
  ];
  exportOutData.value.invalidSample = [
    ...ObjectList_to_2dList(sample, key_en_s, key_cn_s),
    [],
    [],
    ...ObjectList_to_2dList(result, key_en_r, key_cn_r),
  ];
}
// 阳性率
function openPositiveRateDialog() {
  tongJiDialogTitle.value = '阳性率选项设置';
  positiveRateOption.value.isShow = true;
  tongJiSettingDialog.value = true;
}
//样本阳性率获取
async function getPositiveRateData() {
  if (queryParams.value.hospital_id == undefined) {
    ElMessage.warning('请先选择医院后再操作');
    return;
  }

  const qp = {
    options: {
      by_hospital: tongJiQueryParams.value.byHospital,
      by_instrument: tongJiQueryParams.value.byInstrument,
      by_test_group: tongJiQueryParams.value.byTestGroup,
      by_regent_lot: tongJiQueryParams.value.byRegentLot,
      by_test_name: tongJiQueryParams.value.byTestName,
      time_option: tongJiQueryParams.value.byMonth ? 'M' : undefined,
    },
    optString: positiveRateOption.value.optionValue,
    req: {
      ...queryParams.value,
    },
  };
  const [res, msg] = await invoke('get_positive_rate', { ...qp });
  if (msg !== null) {
    ElMessage.error(msg);
    return;
  }
  const data = res.list;
  const title = res.title;

  const key_cn_s = [
    '月份',
    '医院',
    '仪器',
    '试剂',
    '试剂批号',
    '总测试',
    '样本测试',
    '测试名称',
    '阳性率',
  ];
  const key_en_s = [
    'month',
    'hospital',
    'instrument',
    'test_group',
    'regent_lot',
    'all_total',
    'sample_total',
    'test_name',
    'positive%',
  ];
  const key_cn = [
    ...key_cn_s,
    ...title,
    '开始时间',
    '结束时间',
    '阳性数',
    '阳性率',
  ];
  const key_en = [
    ...key_en_s,
    ...title,
    'begin_time',
    'end_time',
    'positive',
    'positive%',
  ];

  exportOutData.value.positiveRateData = ObjectList_to_2dList(
    data,
    key_en,
    key_cn
  );
}

// 磁片统计
//打开磁片设置对话框
function openMagnetSettingDialog() {
  CiPianOptions.value.isShow = true;
  tongJiDialogTitle.value = '磁片统计设置';
  tongJiSettingDialog.value = true;
}
// 磁片统计
async function getCiPianData() {
  if (queryParams.value.hospital_id == undefined) {
    ElMessage.warning('请先选择医院后再操作');
    return;
  }

  const qp = {
    options: {
      by_hospital: tongJiQueryParams.value.byHospital,
      by_instrument: tongJiQueryParams.value.byInstrument,
      by_test_group: tongJiQueryParams.value.byTestGroup,
      by_regent_lot: tongJiQueryParams.value.byRegentLot,
      by_test_name: tongJiQueryParams.value.byTestName,
      time_option: tongJiQueryParams.value.byMonth ? 'M' : undefined,
    },
    optString: CiPianOptions.value.optionValue,
    req: {
      ...queryParams.value,
    },
  };
  const [res, msg] = await invoke('get_cipian_count', { ...qp });
  if (msg !== null) {
    ElMessage.error(msg);
    return;
  }
  const data = res.list;
  const title = res.title;

  const key_cn_s = [
    '月份',
    '医院',
    '仪器',
    '试剂',
    '试剂批号',
    '总测试',
    '测试名称',
    '磁片均值',
  ];
  const key_en_s = [
    'month',
    'hospital',
    'instrument',
    'test_group',
    'regent_lot',
    'all_total',
    'test_name',
    'avg',
  ];
  const key_cn = [
    ...key_cn_s,
    ...title,
    '无效数',
    '百分比',
    '开始时间',
    '结束时间',
    '均值',
    '标准差',
    'CV',
  ];
  const key_en = [
    ...key_en_s,
    ...title,
    'invalid_all',
    'invalid_all_percent',
    'begin_time',
    'end_time',
    'avg',
    'std',
    'cv',
  ];

  exportOutData.value.CiPianCountData = ObjectList_to_2dList(
    data,
    key_en,
    key_cn
  );
}
// 本底范围
// 打开本底设置对话框
function openBenDiRangeSettingDialog() {
  tongJiDialogTitle.value = '本底范围统计设置';
  BendiOptions.value.isShow = true;
  tongJiSettingDialog.value = true;
}
// 本底范围统计
async function getBenDiRange() {
  if (queryParams.value.hospital_id == undefined) {
    ElMessage.warning('请先选择医院后再操作');
    return;
  }
  const qp = {
    options: {
      by_hospital: tongJiQueryParams.value.byHospital,
      by_instrument: tongJiQueryParams.value.byInstrument,
      by_test_group: tongJiQueryParams.value.byTestGroup,
      by_regent_lot: tongJiQueryParams.value.byRegentLot,
      by_test_name: tongJiQueryParams.value.byTestName,
      time_option: tongJiQueryParams.value.byMonth ? 'M' : undefined,
    },
    optString: BendiOptions.value.optionValue,
    req: {
      ...queryParams.value,
    },
  };

  const [[{ result, title }, msg1], [range, msg2]] = await Promise.all([
    invoke('get_bendi_count', { ...qp }),
    invoke('get_bendi_range', { ...qp }),
  ]);
  if (msg1 !== null) {
    ElMessage.error(msg1);
    return;
  }
  if (msg2 !== null) {
    ElMessage.error(msg2);
    return;
  }
  const key_cn_s = ['月份', '医院', '仪器', '试剂', '试剂批号', '测试名称'];
  const key_en_s = [
    'month',
    'hospital',
    'instrument',
    'regent_lot',
    'test_group',
    'test_name',
  ];
  const key_cn_a = [
    ...key_cn_s,
    ...title,
    '开始时间',
    '结束时间',
    '样本均值',
    '总均值',
    '标准差',
    'CV',
  ];
  const key_en_a = [
    ...key_en_s,
    ...title,
    'begin_time',
    'end_time',
    's_avg',
    'all_avg',
    'all_std',
    'all_cv',
  ];

  const key_cn_r = [
    ...key_cn_s,
    '开始时间',
    '结束时间',
    '样本均值',
    '样本范围',
    '校准2范围',
    '校准1范围',
    '对照范围',
    '质控范围',
    '均值',
    '标准差',
    'CV',
    '样本均值',
    '样本标准差',
    '样本CV',
  ];
  const key_en_r = [
    ...key_en_s,
    'begin_time',
    'end_time',
    's_avg',
    's_range',
    'cal2_range',
    'cal1_range',
    'npc_range',
    'qc_range',
    'all_avg',
    'all_std',
    'all_cv',
    's_avg',
    's_std',
    's_cv',
  ];

  exportOutData.value.BenDiRangeData = [
    ...ObjectList_to_2dList(range, key_en_r, key_cn_r),
    [],
    [],
    ...ObjectList_to_2dList(result, key_en_a, key_cn_a),
  ];
}

// 结果数据导出
// 设置对话框
function OpenResultSettingDialog() {
  resultSettingTitle.value = '结果数据导出设置';
  openResultSettingDialog.value = true;
}
async function getAllSampleData() {
  if (queryParams.value.hospital_id == undefined) {
    ElMessage.warning('请先选择医院后再操作');
    return;
  }
  const [{ list, test_names }, msg] = await invoke('get_all_result', {
    req: queryParams.value,
  });
  exportOutData.value.allSample = transfor_sample_result(
    list,
    test_names,
    resultOptionValue.value
  );
}

//  数据页面跳转
const goToData = (id) => {
  proxy.$router.push({ path: "/sample_result", query: { id } });
}


// --
const init = async () => {
  await getHospitalsOption();
  await getList();
};

init();
</script>
<style lang="scss" scoped>
.pagination-container {
  margin-bottom: 20px !important;
}

table {
  border-collapse: collapse;
  border-spacing: 0;
  margin: 1px;
  width: 540px;
}

td,
th {
  padding: 0;
  text-align: center;
}

.pure-table {
  border-collapse: collapse;
  border-spacing: 0;
  empty-cells: show;
  border: 1px solid #cbcbcb;
}

.pure-table caption {
  color: #000;
  font: italic 55%/1 arial, sans-serif;
  padding: 1em 0;
  text-align: center;
}

.pure-table td,
.pure-table th {
  border-left: 1px solid #cbcbcb;
  border-width: 0 0 0 1px;
  font-size: inherit;
  margin: 0;
  overflow: visible;
  padding: 0.5em 1em;
}

.pure-table thead {
  background-color: #e0e0e0;
  color: #000;
  text-align: left;
  vertical-align: bottom;
}

.pure-table td {
  background-color: transparent;
}

.pure-table-bordered td {
  border-bottom: 1px solid #cbcbcb;
}

.pure-table-bordered tbody>tr:last-child>td {
  border-bottom-width: 0;
}

.pure-table-odd td {
  background-color: #dbbdbd64;
}

html.dark {
  .pure-table-odd td {
    background-color: #262323d3;
  }

  .pure-table th {
    background-color: rgba(34, 31, 31, 0.999);
    color: white;
  }
}
</style>
