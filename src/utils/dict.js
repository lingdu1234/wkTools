import { toRefs, ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
/**
 * 获取字典数据
 */
export function useDict(...args) {
    const res = ref({});
    return (() => {
        args.forEach((d, index) => {
            res.value[d] = [];
            invoke('get_dict_data_by_type', { dictType: d  }).then(resp => {
                let [res_data,msg] = resp;
                if (msg == null) {
                    res.value[d] = res_data.map(p => ({ label: p.dict_label, value: p.dict_value, elTagType: p.list_class, status: p.status }));
                }

            })
        })
        return toRefs(res.value);
    })()
}