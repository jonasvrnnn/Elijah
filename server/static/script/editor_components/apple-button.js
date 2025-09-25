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

    document.addEventListener("DOMContentLoaded", () => {
      htmx.process(this.shadowRoot);
    })
  }

  connectedCallback() {
    this.render();
  }

  render() {
    this.shadowRoot.innerHTML = `
      <link rel="stylesheet" href="/components/apple-button.css">
      <div class="background" ></div>
      <a href="${this.getAttribute('href')}" hx-boost="true" >
        <span>${this.textContent.trim()}</span>
        <div class="circle"></div>
      </a>`;
  }
}

// Define the custom element
customElements.define('apple-button', AppleButton);
