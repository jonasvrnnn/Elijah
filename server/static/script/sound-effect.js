function add_click_sound_effect(element, src) {
    let audio = new Audio(src);
    
    element.addEventListener("click", () => {
        audio.play();
    })
}