class Component {
  constructor(name, element) {
    this.name = name;
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
    if (this.element.classList.contains('installed')) {
      return { name: this.name, state: 'installed' };
    } else if (this.element.classList.contains('progress')) {
      return { name: this.name, state: 'progress' };
    } else {
      return { name: this.name, state: 'uninstalled' };
    }
  }
}
