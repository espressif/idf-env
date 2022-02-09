app.component('install-component', {
  template: `
    <div>
      Installing...
    </div>
  `,
  store,
  props: {},
  created() {
    this.sendDesiredState();
  },
  methods: {
    switchInstallTab: function (installTab) {
      this.$store.commit('switchModifyTab', installTab);
    },
    sendDesiredState: function () {
      var workload = this.$store.state.workloads[1];
      var components = workload.components;
      for (var index = 0; index < components.length; index++) {
        var component = components[index];
        if (component.state === component.desiredState) {
          console.log("* Skipping: " + component.id);
          continue;
        }
        console.log("* Requesting: " + component.id + " to become " + component.desiredState);
        window.rmi(JSON.stringify({cmd: "setComponentDesiredState", "name": component.id, "state": component.desiredState}));
      }
    }
  }
});

