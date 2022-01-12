const store = new Vuex.Store({
  state: {
      isInstalledActive: false,
      isAvaialbleActive: false,
      isWorkloadsActive: true,
      isComponentsActive: false,
      isLocationsActive: false
  },
  mutations: {
      switchInstallTab: function (state, installTab) {
          state.isInstalledActive = (installTab === 'installed');
          state.isAvaialbleActive = (installTab === 'available');
      },
      switchModifyTab: function (state, installTab) {
          state.isWorkloadsActive = (installTab === 'workloads');
          state.isComponentsActive = (installTab === 'components');
          state.isLocationsActive = (installTab === 'locations');
      }

  }
});
