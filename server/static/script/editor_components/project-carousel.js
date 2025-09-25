class ProjectCarousel extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {

  }
}
    
customElements.define(
  "project-carousel",
  ProjectCarousel,
  { extends: "textarea" }
);
