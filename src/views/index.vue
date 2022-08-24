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
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { resourceDir } from '@tauri-apps/api/path';
import { resolveResource } from '@tauri-apps/api/path';


const v = ref("");
const v2 = ref("");
const v3 = ref("");
const v4 = ref("");

// 切换语言
async function get_res_path() {
const resourceDirPath = await resourceDir();
const resourcePath = await resolveResource('__data/database/database.db');
const db_blank = await resolveResource('__data/database/db_blank.db');
const db_sql = await resolveResource('__data/sql');
v.value = resourceDirPath;
v2.value = resourcePath;
v3.value = db_blank;
v4.value = db_sql;
}

get_res_path()

</script>