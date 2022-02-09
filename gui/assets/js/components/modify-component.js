app.component('modify-component', {
  template: `
    <div>
      <div>
        <div class="installation-tabs">
          <div class="installation-title"
               v-on:click="switchInstallTab('workloads')"
               :class="{ active: isWorkloadsActive }"
          >Workloads
          </div>
          <!--                <div class="installation-title"-->
          <!--                    v-on:click="switchInstallTab('components')"-->
          <!--                    :class="{ active: isComponentsActive }"-->
          <!--                    >Individual Components</div>-->
          <!--                <div class="installation-title"-->
          <!--                    v-on:click="switchInstallTab('locations')"-->
          <!--                    :class="{ active: isLocationsActive }"-->
          <!--                    >Installation Locations</div>-->
  
          <div class="installation-title-tail"></div>
        </div>
        <hr>
  
        <div class="install-item"
             :class="{ activeitem: isLocationsActive }">
          <div>ESP-IDF</div>
          <input class="location-path" type="text" value="/Users/epsressifer/Desktop/ESP-IDF">
        </div>
  
        <div class="install-item"
             :class="{ activeitem: isLocationsActive }">
          <div>Download cache</div>
          <input class="location-path" type="text" value="/Users/epsressifer/.espressif/dist">
        </div>
  
        <div class="install-item"
             :class="{ activeitem: isLocationsActive }">
          <div>Shared tools</div>
          <input class="location-path" type="text" value="/Users/epsressifer/.espressif/tools">
        </div>
  
        <!--            <div class="workloads-container"-->
        <!--            :class="{ activeitem: isWorkloadsActive }"-->
        <!--            >-->
        <!--              <div class="install-item"-->
        <!--              :class="{ activeitem: isWorkloadsActive }"-->
        <!--              v-for="workload in workloads">-->
        <!--                  -->
        <!--              <h2>{{ workload.title }}</h2>-->
        <!--              <div>{{ workload.description }}</div>-->
        <!--          </div>-->
  
        <div class="install-component"
             :class="{ activeitem: isWorkloadsActive }"
             v-for="workload in workloads">
          <div>{{ workload.title }}</div>
  
          <div
            v-for="component in workload.components">
            <input type="checkbox" v-on:click="toggleDesiredState(component.id)" v-model="component.desiredState"
              true-value="installed"
              false-value="uninstalled"
              :value="component.id"/> {{ component.title }} - {{ component.state }} -&gt; {{
            component.desiredState }}
          </div>
        </div>
      </div>
  
      <div>
        <div class="install-item"
             :class="{ activeitem: isComponentsActive }">
          <h2>Tools</h2>
          <ul>
            Python
            <ul>
              <li v-for="python in pythonList">{{ python.title }}</li>
            </ul>
            Git
            <ul>
              <li>System 4.3.2</li>
            </ul>
          </ul>
        </div>
  
  
        <div class="modify-buttons">
          <div class="modify-button">
            <router-link to="/" tag="div">
              <div class="install-button">Close</div>
            </router-link>
          </div>
          <div class="modify-button">
            <router-link to="/install" tag="div">
              <div class="install-button">Install</div>
            </router-link>
          </div>
          <div class="modify-button-tail"></div>
        </div>
  
      </div>
    </div>
  `,
  store,
  props: {},
  computed: {
    isWorkloadsActive() {
      return this.$store.state.isWorkloadsActive;
    },
    isComponentsActive() {
      return this.$store.state.isComponentsActive;
    },
    isLocationsActive() {
      return this.$store.state.isLocationsActive;
    },
    workloads() {
      return this.$store.state.workloads;
    }

  },
  created() {
    // this.requestObservedState();
  },
  data: function () {
    return {
      options: {
        onlineMode: 'enabled'
      },
      idfList: DOCUMENTATION_VERSIONS.VERSIONS/*[
        { title: 'IDF 4.3', state: 'installed' },
        { title: 'IDF 4.2', state: '' },
        { title: 'IDF 4.1', state: '' },
        { title: 'IDF master', state: '' }
    ]*/,
      pythonList: [
        {title: 'Embedded 3.9'},
        {title: 'Embedded 3.7'},
        {title: 'System 3.6'}
      ],
      sites: [
        {
          state: 'enabled',
          title: 'Unofficial wheels',
          url: 'https://georgik.rocks/tmp/python/pypi'
        },
        {
          state: 'enabled',
          title: 'Official wheels',
          url: 'https://dl.espressif.com'
        }
      ]
    }
  },
  methods: {
    switchInstallTab: function (installTab) {
      this.$store.commit('switchModifyTab', installTab);
    },
    toggleDesiredState: function (componentId) {
      console.log("Component " + componentId);
      if (componentsController === undefined) {
        return;
      }
      this.$store.commit('toggleComponent', {componentId});
      console.log("State definition changed");
      // componentsController.desiredState = getDesiredState();
    }
  }
});

