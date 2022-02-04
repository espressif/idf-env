const store = new Vuex.Store({
  state: {
    isInstalledActive: false,
    isAvaialbleActive: false,
    isWorkloadsActive: true,
    isComponentsActive: false,
    isLocationsActive: false,
    workloads: workloads,
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
    },
    component: function (state, {componentId, observedState}) {
      let rustComponents = state.workloads[1].components;
      for (var index = 0; index < rustComponents.length; index++) {
        if (rustComponents[index].id === componentId) {
          rustComponents[index].state = observedState;
          if (rustComponents[index].desiredState === "unknown") {
            rustComponents[index].desiredState = observedState;
          }
          console.log("Updating state: " + observedState)
        }
      }
    },
    toggleComponent: function (state, {componentId}) {
      let rustComponents = state.workloads[1].components;
      for (var index = 0; index < rustComponents.length; index++) {
        if (rustComponents[index].id === componentId) {
          if (rustComponents[index].desiredState === 'installed') {
            rustComponents[index].desiredState = 'uninstalled';
          } else {
            rustComponents[index].desiredState = 'installed';
          }
          console.log("Toggle state: " + rustComponents[index].desiredState);
        }
      }
    }


  }
});
