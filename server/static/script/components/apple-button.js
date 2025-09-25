class AppleButton extends HTMLElement {
  constructor() {
    super();

    this.text = "Lees meer";

    this.attachShadow({
      mode: 'open'
    })

    new IntersectionObserver(entries => {
      entries.forEach(entry => entry.target.classList.toggle("active", entry.isIntersecting));
    }, {
      root: null,
      rootMargin: '-120px 0px -120px 0px',
      threshold: .1
    }).observe(this);

    new MutationObserver(mutations => {
      mutations.forEach(mutation => {
        if (mutation.type === 'childList' || mutation.type === 'characterData') {
          this.text = this.textContent.trim();
          this.render();

        }
      });
    }).observe(this, {
      childList: true,
      characterData: true,
      subtree: true
    });
  }

  connectedCallback() {
    this.render();
  }

  render() {
    this.shadowRoot.innerHTML = `
      <style>
        @import url('/components/apple-button.css')
      </style>

      <div class="background" ></div>
      <a href="${this.getAttribute('href')}" hx-boost="true" >
        <span>${this.textContent.trim()}</span>
        <div class="circle">
          <div class="circle-background"></div>
          <svg style="aspect-ratio: 1/1;" xmlns="http://www.w3.org/2000/svg" fill="none"
            viewBox="8.22 5.64 7.78 12.73">
            <path
              d="M13.1719 12L8.22192 7.04999L9.63592 5.63599L15.9999 12L9.63592 18.364L8.22192 16.95L13.1719 12Z"
              fill="CurrentColor"></path>
          </svg>
        </div>
      </a>`;

    htmx.process(this.shadowRoot)
  }

  disconnectedCallback() {
  }

  attributeChangedCallback(name, oldValue, newValue) {
  }

  static get observedAttributes() {
    return ['my-attribute'];
  }
}

// Define the custom element
customElements.define('apple-button', AppleButton);
