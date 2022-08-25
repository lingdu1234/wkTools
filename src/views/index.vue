<template>
  <div>
    <p>首页</p>
    <p>{{v}}</p>
    <p>==========</p>
    <p>{{v2}}</p>
    <p>==========</p>
    <p>{{v3}}</p>
    <p>==========</p>
    <p>{{v4}}</p>
        <p>====appDir======</p>
    <p>{{_appdir}}</p>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { resourceDir,resolveResource,appDir } from '@tauri-apps/api/path';


const v = ref("");
const v2 = ref("");
const v3 = ref("");
const v4 = ref("");
const _appdir = ref("");

// 切换语言
async function get_res_path() {
const resourceDirPath = await resourceDir();
const resourcePath = await resolveResource('__data/database/database.db');
const db_blank = await resolveResource('__data/database/db_blank.db');
const db_sql = await resolveResource('__data/sql');
const appDirPath = await appDir();
v.value = resourceDirPath;
v2.value = resourcePath;
v3.value = db_blank;
v4.value = db_sql;
_appdir.value = appDirPath;

}

get_res_path()

</script>