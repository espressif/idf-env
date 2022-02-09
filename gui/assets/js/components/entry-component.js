app.component('entry-component', {
  template: `
  <div>
      <div class="installation-tabs">
        <div class="installation-title"
             v-on:click="switchInstallTab('installed')"
             :class="{ active: isInstalledActive }"
        >Installed
        </div>
        <div class="installation-title"
             v-on:click="switchInstallTab('available')"
             :class="{ active: isAvaialbleActive }"
        >Available
        </div>
        <div class="installation-title-tail"></div>
      </div>
      <hr>
      <input type="text" value="Search for chip">

      <div class="install-item"
           :class="{ activeitem: isInstalledActive }"
           v-for="xidf in idfList2">
        <div class="install-item-left">
          <div class="esp-logo"></div>
        </div>
        <div class="install-item-middle">
          <div>ESP-IDF v4.3</div>
          <div class="install-item-info">Release date: 2020-09-01</div>
          <div class="install-item-info">Chips: ESP32, ESP32-S2</div>
          <div class="install-item-info">Period:
            <ul>
              <li>Service period (recommended for new designs) until 2021-10</li>
              <li>Maintenance period 2023-05</li>
            </ul>
          </div>
          <div>
<!--            <router-link to="release-notes">Release notes</router-link>-->
          </div>
        </div>
        <div class="install-item-right">
          <router-link to="/modify">
            <div class="install-button">Modify</div>
          </router-link>
          <div class="install-button">Launch</div>
          <div class="install-button">More</div>
        </div>
        <div class="install-item-tail"></div>
      </div>

      <div class="install-item"
           :class="{ activeitem: isAvaialbleActive }">
        <div class="install-item-left">
          <div class="esp-logo"></div>
        </div>
        <div class="install-item-middle">
          <div>ESP-IDF 4.3</div>
          <div class="install-item-info">Release date: 2021-05-01</div>
          <div class="install-item-info">Chips: ESP32, ESP32-S2</div>
          <div class="install-item-info">Period:
            <ul>
              <li>Development period until 2021-05</li>
              <li>Service period TBD</li>
              <li>Maintenance period TBD</li>
            </ul>
          </div>
          <div>
<!--            <router-link to="/license">License terms</router-link>-->
            |
<!--            <router-link to="release-notes">Release notes</router-link>-->
          </div>
        </div>
        <div class="install-item-right">
          <div class="install-button">Install</div>
          <div class="install-button">More</div>
        </div>

        <div class="install-item-tail"></div>
      </div>

      <div class="install-item"
           :class="{ activeitem: isAvaialbleActive }">
        <div class="install-item-left">
          <div class="esp-logo"></div>
        </div>
        <div class="install-item-middle">
          <div>ESP-IDF 3.3.2</div>
          <div class="install-item-info">Release date: 2019-09-01</div>
          <div class="install-item-info">Chips: ESP8266</div>
          <div class="install-item-info">Period:
            <ul>
              <li>Maintenance period until 2021-12</li>
            </ul>
          </div>
          <div>
<!--            <router-link to="/license">License terms</router-link>-->
            |
<!--            <router-link to="release-notes">Release notes</router-link>-->
          </div>
        </div>
        <div class="install-item-right">
          <div class="install-button">Install</div>
          <div class="install-button">More</div>
        </div>

        <div class="install-item-tail"></div>
      </div>
    </div>
  `,
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

