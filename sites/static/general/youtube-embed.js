class YoutubeEmbed extends HTMLElement {
  constructor() {
    super();

    const iframe = document.createElement("iframe");

    iframe.loading = "lazy";
    iframe.frameBorder = 0;
    iframe.allow = "accelerometer; autoplay playsinline; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share";
    iframe.referrerPolicy = "strict-origin-when-cross-origin";
    iframe.toggleAttribute("allowfullscreen", true);

    this.iframe = iframe;
  }

  connectedCallback() {
    const id = this.getAttribute("yt-id");
    
    this.iframe.src  = `https://www.youtube.com/embed/${id}`;

    this.replaceWith(this.iframe);
  }
}
    
customElements.define(
  "yt-embed",
  YoutubeEmbed
);
