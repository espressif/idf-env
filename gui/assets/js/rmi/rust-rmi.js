/**
 * RMI Wrapper to route requests to Rust core or emulate it in browser.
 */
if (window.external.hasOwnProperty('invoke')) {
  window.rmi = window.external.invoke;
} else {
  window.rmi = function(value) {
    console.log("RMI emulation - call: " + value);
  };
}