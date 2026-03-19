import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/providers',
    },
    {
      path: '/projects',
      name: 'projects',
      component: () => import('../pages/projects/index.vue'),
    },
    {
      path: '/projects/add',
      name: 'projects-add',
      component: () => import('../pages/projects/add.vue'),
    },
    {
      path: '/projects/:id/edit',
      name: 'projects-edit',
      component: () => import('../pages/projects/edit.vue'),
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
