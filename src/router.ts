import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      component: () => import("@/views/Login.vue"),
    },
    // {
    //   path: "/dashboard",
    //   component: () => import("@/views/Dashboard.vue"),
    // },
    {
      path: "/",
      component: () => import("@/layout/index.vue"),
      children: [
        {
          path: "page/:pageId",
          component: () => import("@/layout/page.vue"),
          children: [
            {
              path: "home",
              component: () => import("@/views/Home.vue"),
            },

          ]
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
    },
  ],
});

export default router;
