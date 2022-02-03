class Component {
  constructor(id, element) {
    this.id = id;
    this.element = element;
    this.isBusy = false;
  }

  add() {
    if (this.isBusy) {
      return;
    }
    if (this.element.classList.contains('installed')) {
      return;
    }
    this.isBusy = true;
    this.element.classList.add("progress");
    var element = this.element;
    var self = this;
    const timerId = setTimeout(function () {
      element.classList.remove("progress")
      element.classList.add("installed");
      self.isBusy = false;
      clearTimeout(timerId);
    },1500);
  }

  remove() {
    if (this.isBusy) {
      return;
    }
    if (!this.element.classList.contains('installed')) {
      return;
    }
    this.isBusy = true;
    this.element.classList.remove("installed");
    this.isBusy = false;
  }

  observe() {
    window.external.invoke(JSON.stringify({cmd:"getComponentStatus", "name":this.id}));
    if (this.element.classList.contains('installed')) {
      return { id: this.id, state: 'installed' };
    } else if (this.element.classList.contains('progress')) {
      return { id: this.id, state: 'progress' };
    } else {
      return { id: this.id, state: 'uninstalled' };
    }
  }
}
