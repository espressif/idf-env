let entryComponent = Vue.component('entry-component', {
  template: '#entry-component-template',
  store,
  props: {},
  computed: {
    isInstalledActive() {
      return this.$store.state.isInstalledActive;
    },
    isAvaialbleActive() {
      return this.$store.state.isAvaialbleActive;
    }
  },
  data: function () {
    return {
      options: {
        onlineMode: 'enabled'
      },
      idfList2: DOCUMENTATION_VERSIONS.VERSIONS/*[
            { title: 'IDF 4.3', state: 'installed' },
            { title: 'IDF 4.2', state: '' },
            { title: 'IDF 4.1', state: '' },
            { title: 'IDF master', state: '' }
        ]*/
    }
  },

  methods: {
    switchInstallTab: function (installTab) {
      this.$store.commit('switchInstallTab', installTab);
      //window.external.notify("aa");
    }
  }
});

