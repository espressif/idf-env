const routes = [
  { path: '/', component: entryComponent },
  { path: '/modify', component: modifyComponent },
  { path: '/install', component: installComponent }
];

const router = new VueRouter({
  routes // short for `routes: routes`
});
