var DOCUMENTATION_VERSIONS = {
	    DEFAULTS: { has_targets: false,
		                    supported_targets: [ "esp32" ]
		                  },
	    VERSIONS: [
		            { name: "latest", has_targets: true, supported_targets: [ "esp32", "esp32s2" ] },

		            { name: "v4.3-beta1", pre_release: true, old:false, has_targets: true, supported_targets: [ "esp32", "esp32s2", "esp32c3" ]},

		            { name: "v4.2", old: false, has_targets: true, supported_targets: [ "esp32", "esp32s2" ]},

		            { name: "v4.1.1", old: false },
		            { name: "v4.1", old: true },

		            { name: "v4.0.2", old: false},
		            { name: "v4.0.1", old: true},
		            { name: "v4.0", old: true },

		            { name: "v3.3.4", old: false },
		            { name: "v3.3.3", old: true },
		            { name: "v3.3.2", old: true },
		            { name: "v3.3.1", old: true },
		            { name: "v3.3", old: true  },

		            { name: "v3.2.5", old: true },

		            { name: "v3.1.7", old: true },

		            { name: "release-v4.3", pre_release: true, has_targets: true, supported_targets: [ "esp32", "esp32s2", "esp32c3" ]},
		            { name: "release-v4.2", pre_release: true, has_targets: true, supported_targets: [ "esp32", "esp32s2" ] },
		            { name: "release-v4.1", pre_release: true },
		            { name: "release-v4.0", pre_release: true },
		            { name: "release-v3.3", pre_release: true }
		        ],
	    IDF_TARGETS: [
		           { text: "ESP32", value: "esp32"},
		           { text: "ESP32-S2", value: "esp32s2"},
		           { text: "ESP32-C3", value: "esp32c3"}
		        ],
	   RELEASES: {
		           "v3.1": {
				               "start_date": "2018-09-07",
				               "end_date": "2020-10-31"
				           },
		           "v3.2": {
				               "start_date": "2019-04-11",
				               "end_date": "2020-10-31"
				           },
		           "v3.3": {
				               "start_date": "2019-09-05",
				               "end_date": "2022-02-28"
				           },
		           "v4.0": {
				               "start_date": "2020-02-11",
				               "end_date": "2021-10-31"
				           },
		           "v4.1": {
				               "start_date": "2020-08-24",
				               "end_service": "2021-08-24",
				               "end_date": "2023-02-24"
				           },
		           "v4.2": {
				               "start_date": "2020-12-07",
				               "end_service": "2021-12-07",
				               "end_date": "2023-06-07"
				           }
		       }
};

