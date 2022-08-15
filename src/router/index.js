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
  routes: redirectRoute.concat(...constantRoutes),
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return { top: 0 }
    }
  },
});

export default router;
