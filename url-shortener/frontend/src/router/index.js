import { createRouter, createWebHistory } from "vue-router";
import URLShortener from "../components/URLShortener.vue";
import URLRedirect from "../components/URLRdirect.vue";

const routes = [
  {
    path: "/:shortCode",
    name: "Redirect",
    component: URLRedirect,
  },
  {
    path: "/",
    name: "Home",
    component: URLShortener,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
