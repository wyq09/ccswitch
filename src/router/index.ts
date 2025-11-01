import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/providers',
    },
    {
      path: '/providers',
      name: 'providers',
      component: () => import('../pages/providers/index.vue'),
    },
    {
      path: '/providers/add',
      name: 'providers-add',
      component: () => import('../pages/providers/add.vue'),
    },
    {
      path: '/providers/:id/edit',
      name: 'providers-edit',
      component: () => import('../pages/providers/edit.vue'),
    },
  ],
})

export default router

