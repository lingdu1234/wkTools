import { createWebHashHistory, createRouter } from 'vue-router'

import i18n from '@/locals'

const { t } = i18n.global

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
    path: '/dict_data/:dict_type_id',
    component: () => import('@/views/dict/data'),
    name: 'Dict_Data',
  }
]

// 公共路由
export const constantRoutes = [

  {
    path: '/index',
    component: () => import('@/views/index'),
    name: "Index",
    meta: { title: t("route.home"), icon: 'home' },

  },
  {
    path: '/img',
    component: () => import('@/views/img'),
    name: "Img",
    meta: { title: t("route.img"), icon: 'img' },

  },
  {
    path: '/excel',
    component: () => import('@/views/excel'),
    name: "Excel",
    meta: { title: t("route.excel"), icon: 'excel' },

  },
  {
    path: '/dict',
    component: () => import('@/views/dict'),
    name: "Dict",
    meta: { title: t("route.dict"), icon: 'dict' },
  },
  {
    path: '/regent',
    component: () => import('@/views/regent'),
    name: "Regent",
    meta: { title: t("route.regent"), icon: 'regent' },

  },
  {
    path: '/setting',
    component: () => import('@/views/setting'),
    name: 'Setting',
    meta: { title: t("route.setting"), icon: 'setting' }
  },
  {
    path: '/about',
    component: () => import('@/views/about'),
    name: 'About',
    meta: { title: t("route.about"), icon: 'about' }
  },
];

const router = createRouter({
  // history: createWebHistory(),
  history: createWebHashHistory(),
  routes: redirectRoute.concat(...hiddenRoute,...constantRoutes),
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return { top: 0 }
    }
  },
});

export default router;
