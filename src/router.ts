import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      component: () => import("@/views/Login.vue"),
    },
    {
      path: "/dashboard",
      component: () => import("@/views/Dashboard.vue"),
    },
    {
      path: "/",
      redirect: "/dashboard",
    },
  ],
});

export default router;
