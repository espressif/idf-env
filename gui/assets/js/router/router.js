const routes = [
  // {path: '/', component: entryComponent},
  // {path: '/modify', component: modifyComponent},
  // {path: '/install', component: installComponent}
];

const router = new VueRouter.createRouter({
  history: VueRouter.createWebHashHistory(),
  routes // short for `routes: routes`
});
