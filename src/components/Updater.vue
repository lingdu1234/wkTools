<template>
    <el-dialog :model-value="props.is_show" width="500px" :show-close="false" :before-close="dialogclose">
        <template #header>
            <div class="update-header">
                <div class="up_logo">
                    <img src="@/assets/logo2.png" />
                    <span>程序更新</span>
                </div>
            </div>
            <el-divider style="margin: 5px auto 0;"></el-divider>
        </template>
        <el-form ref="updateRef" label-width="80px">
            <el-form-item label="当前版本">
                <span>{{ current_version }}</span>
            </el-form-item>
            <el-form-item label="更新版本">
                <span>{{ props.up_info.version }}</span>
            </el-form-item>
            <el-form-item label="更新日期">
                <span>{{ props.up_info.date }}</span>
            </el-form-item>
            <el-form-item label="更新日志" >
                <el-scrollbar height="300px">
                    <span>{{ props.up_info.body }}</span>
                </el-scrollbar>
            </el-form-item>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="submitForm">确 定</el-button>
                <el-button @click="dialogclose">取 消</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { getVersion } from '@tauri-apps/api/app';
import { installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import { ElMessage } from 'element-plus';

const props = defineProps({
    up_info: {
        type: Object,
        required: true
    },
    is_show: {
        type: Boolean,
        required: true,
        default: false
    },
})


const emits = defineEmits(["closeUpdateDialog"]);

const current_version = ref("");

const get_base_info = async () => {
    current_version.value = await getVersion();
};


const submitForm = async () => {
    try {
        ElMessage.info("开始更新并重启")
        await installUpdate();
        await relaunch();
    } catch (err) {
        ElMessage.error("更新出错:" + err)
        emits("closeUpdateDialog")
    }
}

const dialogclose = () => {
    ElMessage.info("你取消了更新")
    emits("closeUpdateDialog")
}

get_base_info()
</script>

<style lang="scss" scoped>
html.dark {
    .up_logo {
        span {
            color: #FFFFFF;
        }
    }
}

.el-form {
    font-weight: bolder;

    span {
        font-weight: normal;
        word-break: normal;
        width: 380px;
        display: block;
        white-space: pre-wrap;
        word-wrap: break-word;
        overflow: hidden;
    }
}

.update-header {
    height: 30px;
    color: black;
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0px;

    .up_logo {
        display: flex;
        justify-content: start;

        img {
            height: 22px;
            width: 22px;
        }

        span {
            margin-left: 10px;
            font-size: 18px;
        }
    }
}
</style>