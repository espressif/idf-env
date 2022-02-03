let installComponent = Vue.component('entry-component', {
  template: '#install-component-template',
  store,
  props: {},
  created() {
    window.rmi(JSON.stringify({cmd: "setComponentDesiredState", "name": "rustup", "state": "installed"}));
  },
  methods: {
    switchInstallTab: function (installTab) {
      this.$store.commit('switchModifyTab', installTab);
    }
  }
});

