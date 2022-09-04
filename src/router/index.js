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
