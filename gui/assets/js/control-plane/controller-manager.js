
class ControllerManager {
  constructor() {
    this.controllers = [];
  }

  reconcile() {
    this.controllers.forEach(function (controller) {
      controller.observe();
      controller.reconcile();
    });
  }

  addController(controller) {
    this.controllers.push(controller);
  }

  loop() {
    setInterval(() => {
      this.reconcile();
    }, 1000);
  }
}
