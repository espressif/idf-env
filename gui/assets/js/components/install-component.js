let installComponent = Vue.component('entry-component', {
  template: '#install-component-template',
  store,
  props: {},
  created() {
    external.invoke('install')
  },
  methods: {
    switchInstallTab: function (installTab) {
      this.$store.commit('switchModifyTab', installTab);
    }
  }
});

