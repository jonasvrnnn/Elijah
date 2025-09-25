class AutoSize extends HTMLTextAreaElement {
  constructor() {
    super();
    autosize(this);
  }

  connectedCallback() {
    setTimeout(() => {
        autosize.update(this);
    }, 500);
  }
}
    
customElements.define(
  "auto-size",
  AutoSize,
  { extends: "textarea" }
);
