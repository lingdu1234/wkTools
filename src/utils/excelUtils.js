// import * as XLSX from 'xlsx/xlsx.mjs';
import * as XLSX from 'xlsx';

/**
 * 处理单个`Excel`文件
 * @file 作为参数
 * @raw 是否原始数据 true，为原始数据，false为转换后的数据String
 * @header 1 生成二维数组，‘A’ 生成 对象数组
 * @returns [[]]
 */
export const readExcel = (file, raw, header) => {
  return new Promise((resolve, _) => {
    // 数据处理
    let data
    const _excelReader = new FileReader()
    _excelReader.readAsArrayBuffer(file)
    _excelReader.onload = function (e) {
      let _binary = ''
      // @ts-ignore
      const _bytes = new Uint8Array(e.target.result)
      const _length = _bytes.byteLength
      for (let i = 0; i < _length; i++) {
        _binary += String.fromCharCode(_bytes[i])
      }
      // 读取工作表
      const wb = XLSX.read(_binary, { type: 'binary' })
      // 选择第一个sheet表
      const _sheetName = wb.SheetNames[0]
      const _workSheet = wb.Sheets[_sheetName]
      // excel转换json数组,加上{header:1}是普通数组，不写是对象数组
      data = XLSX.utils.sheet_to_json(_workSheet, {
        header: header, raw: raw
      })
      resolve(data)
    }
  })
}

/**
 * 将`MAP`结果的`json`数据导出为Excel表格
 */
// export const exportJsonData2excel = (exceldata) => {
//   const wb = XLSX.utils.book_new() // 创建数据表

//   exceldata.forEach((v, k) => {
//     const worksheet = XLSX.utils.json_to_sheet(v)
//     XLSX.utils.book_append_sheet(wb, worksheet, k)
//   })
//   XLSX.writeFile(wb, `sampleData.xlsx`) // 下载写入文件
// }

// export const exportSingleJsonData2excel = (exceldata, header, fileName) => {
//   const wb = XLSX.utils.book_new() // 创建数据表
//   const worksheet = XLSX.utils.json_to_sheet(exceldata, { header: header })
//   XLSX.utils.book_append_sheet(wb, worksheet, fileName)
//   XLSX.writeFile(wb, fileName + '.xlsx') // 下载写入文件
// }

/**
 * 将`MAP`结果的`List`数据导出为Excel表格
 */
// export const exportListData2excel = (exceldata, filename) => {
//   const wb = XLSX.utils.book_new() // 创建数据表
//   exceldata.forEach((v, k) => {
//     const worksheet = XLSX.utils.aoa_to_sheet(v)
//     XLSX.utils.book_append_sheet(wb, worksheet, k)
//   })
//   XLSX.writeFile(wb, filename + '.xlsx') // 下载写入文件
// }

/**
 * 将单个的`List`数据导出为Excel表格
 */
// export const exportSingleListData2excel = (exceldata, filename) => {
//   const wb = XLSX.utils.book_new() // 创建数据表
//   const worksheet = XLSX.utils.aoa_to_sheet(exceldata)
//   XLSX.utils.book_append_sheet(wb, worksheet, filename)
//   XLSX.writeFile(wb, filename + '.xlsx') // 下载写入文件
// }

/**
 * 将`map`中的`List`数据导出为CSV
 */
// export const exportListData2csv = (exceldata, key, filename) => {
//   // const wb = XLSX.utils.book_new() // 创建数据表
//   const worksheet = XLSX.utils.aoa_to_sheet(exceldata.get(key))
//   const csv = XLSX.utils.sheet_to_csv(worksheet)
//   //定义文件内容，类型必须为Blob 否则createObjectURL会报错
//   let content = new Blob([csv])
//   //生成url对象
//   let urlObject = window.URL || window.webkitURL || window
//   let url = urlObject.createObjectURL(content)
//   //生成<a></a>DOM元素
//   let el = document.createElement('a')
//   //链接赋值
//   el.href = url
//   el.download = filename + '.csv'
//   //必须点击否则不会下载
//   el.click()
//   //移除链接释放资源
//   urlObject.revokeObjectURL(url)

// }

// export const exportSingleListData2csv = (exceldata, filename) => {
//   // const wb = XLSX.utils.book_new() // 创建数据表
//   const worksheet = XLSX.utils.aoa_to_sheet(exceldata)
//   const csv = XLSX.utils.sheet_to_csv(worksheet)
//   //定义文件内容，类型必须为Blob 否则createObjectURL会报错
//   let content = new Blob([csv])
//   //生成url对象
//   let urlObject = window.URL || window.webkitURL || window
//   let url = urlObject.createObjectURL(content)
//   //生成<a></a>DOM元素
//   let el = document.createElement('a')
//   //链接赋值
//   el.href = url
//   el.download = filename + '.csv'
//   //必须点击否则不会下载
//   el.click()
//   //移除链接释放资源
//   urlObject.revokeObjectURL(url)

// }

// 数组求和
// export const arrSum = (arr) => {
//   var s = 0
//   for (var i = arr.length - 1; i >= 0; i--) {
//     if (arr[i] != null) {
//       s += arr[i]
//     }
//   }
//   return s
// }
