import { createWebHashHistory, createRouter } from 'vue-router'

//  跳转路由
const redirectRoute = [
  {
    path: '',
    redirect: '/index'
  },
]

//  跳转路由
const hiddenRoute = [
  {
    path: '/dict_data/:dict_type',
    component: () => import('@/views/dict/data'),
    name: 'Dict_Data',
  },
  {
    path: '/hospital_ins',
    component: () => import('@/views/hospital/instrument'),
    name: 'Instrument',
  },
  {
    path: '/sample_result',
    component: () => import('@/views/sample/sample_result'),
    name: 'SampleResult',
  }
]

// 公共路由
export const constantRoutes = [

  {
    path: '/index',
    component: () => import('@/views/index'),
    name: "home",
    // meta: { title: t("route.home"), icon: 'home' },
    meta: { title: "route.home", icon: 'home' },

  },
  {
    path: '/img',
    component: () => import('@/views/tools/img'),
    name: "img",
    meta: { title: "route.img", icon: 'img' },

  },
  {
    path: '/excel',
    component: () => import('@/views/tools/excel'),
    name: "excel",
    meta: { title: "route.excel", icon: 'excel' },

  },
  {
    path: '/dict',
    component: () => import('@/views/dict'),
    name: "dict",
    meta: { title: "route.dict", icon: 'dict' },
  },
  {
    path: '/regent',
    component: () => import('@/views/regent'),
    name: "regent",
    meta: { title: "route.regent", icon: 'regent' },

  },
  {
    path: '/hospital',
    component: () => import('@/views/hospital'),
    name: "hospital",
    meta: { title: "route.hospital", icon: 'hospital' },

  },
  {
    path: '/sample',
    component: () => import('@/views/sample'),
    name: "sample",
    meta: { title: "route.sample", icon: 'sample' },

  },
  {
    path: '/statistics',
    component: () => import('@/views/sample/sample_statistics'),
    name: "statistics",
    meta: { title: "route.statistics", icon: 'statistics' },

  },
  // {
  //   path: '/setting',
  //   component: () => import('@/views/setting'),
  //   name: 'Setting',
  //   meta: { title: "route.setting", icon: 'setting' }
  // },
  {
    path: '/about',
    component: () => import('@/views/about'),
    name: 'about',
    meta: { title: "route.about", icon: 'about' }
  },
];

const router = createRouter({
  // history: createWebHistory(),
  history: createWebHashHistory(),
  routes: redirectRoute.concat(...hiddenRoute, ...constantRoutes),
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return { top: 0 }
    }
  },
});

export default router;
