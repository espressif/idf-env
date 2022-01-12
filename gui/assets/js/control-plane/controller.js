
class Controller {
  constructor() {
    this.obsevedState = {};
    this.desiredState = {};
    this.components = [];
  }

  addComponent(component) {
    this.components.push(component);
  }

  getDesiredComponent(componentName) {
    for (var index = 0; index < this.desiredState.components.length; index++) {
      let component = this.desiredState.components[index];
      if (component.name == componentName) {
        return component;
      }
    }
    return undefined;
  }

  reconcile() {
    let desiredStateString = JSON.stringify(this.desiredState);
    let observedSateString = JSON.stringify(this.obsevedState);
    if ((desiredStateString == '{}') ||(desiredStateString === observedSateString)) {
      console.log("Reconcile: NOP");
      return;
    }
    console.log("Reconcile");
    console.log("Observed state: " + observedSateString);
    console.log("Desired sate: " + desiredStateString);
    var self = this;
    
    let desiredComponents = this.desiredState.components
    if (desiredComponents == undefined) {
      return;
    }

    this.components.forEach(function(component) {
      let componentName = component.name;
      
      let desiredComponent = self.getDesiredComponent(componentName);
      if (desiredComponent == undefined) {
        return;
      }

      if (desiredComponent.state == 'installed')  {
        component.add();
      } else if (desiredComponent.state == 'uninstalled') {
        component.remove();
      } else {
        console.log("component update in progress...");
      }
    });
    
  }

  observe() {
    var self = this;
    var componentList = [];
    this.components.forEach(function(component) {
      componentList.push(component.observe());
    });
    self.obsevedState = {components: componentList};
  }
}
