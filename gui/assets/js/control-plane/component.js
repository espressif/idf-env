class Component {
  constructor(id) {
    this.id = id;
    this.state = 'unknown';
    this.desiredState = 'unknown';
  }

  add() {
  }

  remove() {
  }

  observe() {
    window.external.invoke(JSON.stringify({cmd:"getComponentStatus", "name":this.id}));
  }
}
