class TextEditor extends HTMLDivElement {
    constructor() {
        super();

        this.setAttribute("data-fontname", "no");
        this.setAttribute("data-forecolor", "no");
        this.setAttribute("data-justifyleft", "no");
        this.setAttribute("data-justifycenter", "no");
        this.setAttribute("data-justifyright", "no");
        this.setAttribute("data-outdent", "no");
        this.setAttribute("data-indent", "no");
        this.setAttribute("data-autofocus", "no");
    }

    connectedCallback() {
        const create_editor = () => {
            window.__tinyEditor.transformToEditor(this);
            this.parentNode.removeEventListener("click", create_editor);

            document.addEventListener("mousedown", clear_editor);

            this.parentNode.addEventListener("blur", clear_editor);
            this.focus();

            this.addEventListener("paste", e => {
                e.preventDefault();

                const text = e.clipboardData.getData("text");
                const selection = window.getSelection();
                if (!selection.rangeCount) return false;
                selection.deleteFromDocument();
                selection.getRangeAt(0).insertNode(document.createTextNode(text));
            })
        }

        const clear_editor = e => {
            if (e.target !== this.parentNode && !this.parentNode.contains(e.target)) {
                this.previousSibling.remove();
                this.classList.remove("__editor");
                this.parentNode.removeEventListener("blur", clear_editor);
                this.parentNode.addEventListener("click", create_editor);

                const paragraphs = this.querySelectorAll("p:empty");

                for(let paragraph of paragraphs) {
                    paragraph.remove();
                }

                this.parentNode.setAttribute("hx-vals", JSON.stringify({
                    content: this.innerHTML
                }));

                const event = new Event('change');
                this.parentNode.dispatchEvent(event);
            }
        }

        this.parentNode.addEventListener("click", create_editor);
    }
}

// Define the new element
customElements.define('text-editor', TextEditor, {
    extends: "div"
});
