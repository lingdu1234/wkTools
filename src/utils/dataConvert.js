/**
 * 将`Object`数组结果的转为2D`List`数据
 */
export const ObjectList_to_2dList = (data, key_en, key_cn) => {
  const list = [];
  list.push(key_cn);
  data.forEach(v => {
    let row = [];
    key_en.forEach(k => {
      row.push(v[k])
    })
    list.push(row)
  })
  return list;
}


/**
 * 将`Object`数组结果仙童项合并，不同项为children的`Object`数组
 * @param {Array} data，`Object`数组  
 * @param {String[]} commKey，`String`数组的key
 * @param {String[]} childKey，`String`数组的key
 * @param {String[]} time_list`String`数组的key
 * @param {String} key`String`子数组的key
 */
export const ObjectList_to_childList = (data, commKey, childKey, time_list, key) => {
  const result = [];
  const list_key = [];
  let map = {};
  for (let i = 0; i < data.length; i++) {
    const it = data[i];
    let comm_key = '';
    commKey.forEach(cmk => {
      comm_key += it[cmk];
    })


    if (list_key.indexOf(comm_key) == -1) {
      list_key.push(comm_key);
      let itemList = {};
      commKey.forEach(cmk => {
        itemList[cmk] = it[cmk];
        map[comm_key] = [];
      })
      result.push(itemList);
    }

    let ccc = {};
    let t = {}
    childKey.forEach(ck => {
      t[ck] = it[ck]
    })
    ccc[it[key]] = t;
    map[comm_key].push(ccc)
  }

  for (let i = 0; i < list_key.length; i++) {
    let kk = list_key[i];
    result[i]['children'] = map[kk];
  }
  return result;
}

export const childList_to_listlist = (data, commKey, childKey, time_list, children) => {
  const result = [];
  let com_title_one = [];
  let com_title_two = [];
  com_title_one.push(...commKey);
  com_title_two.push(...commKey);
  time_list.forEach(t => {
    childKey.forEach(ck => {
      com_title_one.push(t);
      com_title_two.push(ck);
    })
  })
  result.push(com_title_one);
  result.push(com_title_two);
  data.forEach(v => {
    let itemList = [];
    commKey.forEach(cmk => {
      itemList.push(v[cmk])
    })
    let it_child_list = new Array(time_list.length * childKey.length).fill(null);
    if (v[children]) {

      v[children].forEach(c => {
        time_list.forEach(t => {
          if (c[t]) {
            let index = time_list.indexOf(t)*childKey.length;
            childKey.forEach(ck => {
              if (c[t][ck]) {
                it_child_list[index] = c[t][ck];
              }
              index++;
            })

          }
        })
      })
    }
    itemList.push(...it_child_list);
    result.push(itemList);
  })

  return result;
}

export const transfor_sample_result = (data, test_names, option) => {
  let list = [];
  let title = [
    '医院',
    '仪器',
    '条码',
    '类型',
    '项目组',
    '状态',
    '测试时间',
    '试剂批次',
    '备注',
  ];
  const test_item = [];
  test_names.forEach(t => {
    option.forEach(o => {
      test_item.push(t + '-' + o);
    })
  })
  title.push(...test_item);
  list.push(title);
  for (const it of data) {
    let s = it[0];
    let rs = it[1];
    let list_item = [
      s["hospital_name"], s["instrument_sn"], s["sample_code"], s["sample_type"], s["test_group"], s["status"], s["test_time"], s["regent_lot"], s["remark"],];
    let test_result = new Array(test_item.length).fill(null);
    for (const r of rs) {
      let index_a = test_names.indexOf(r['test_name']);
      for (let i = 0; i < option.length; i++) {
        const e = option[i];
        switch (e) {
          case "N":
            test_result[index_a * option.length + i] = r["result_count"];
            break;
          case "S":
            test_result[index_a * option.length + i] = r["result_signal"];
            break;
          case "A":
            test_result[index_a * option.length + i] = r["result_ai"];
            break;
          case "I":
            test_result[index_a * option.length + i] = r["result_index"];
            break;
        }

      }
    }
    list_item.push(...test_result);
    list.push(list_item);
  }
  return list;
}
