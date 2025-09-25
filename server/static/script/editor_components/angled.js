class AngledElement extends HTMLElement  {
    constructor() {
        super();

        this.angle_top = 0;
        this.angle_bottom = 0;

        this.triangle_top = false;
        this.triangle_bottom = false;

        this.triangle_top_position = "0";
        this.triangle_bottom_position = "0";

        this.triangle_top_size = 0;
        this.triangle_bottom_size = 0;

        this.mobile_landscape_max_width = 767;

        let resizeObserver = new ResizeObserver(() => {
            this.render();
        });

        resizeObserver.observe(this);
    }

    static get observedAttributes() {
        return [
            "angle-top",
            "angle-bottom",
            "triangle-top",
            "triangle-bottom",
            "triangle-top-position",
            "triangle-bottom-position",
            "triangle-top-size",
            "triangle-bottom-size"
        ];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        switch (name) {
            case "angle-top": try {
                this.angle_top = parseFloat(newValue);
            } catch {
                this.angle_top = 0;
            } break;

            case "angle-bottom": try {
                this.angle_bottom = parseFloat(newValue);
            } catch {
                this.angle_bottom = 0;
            } break;

            case "triangle-top": this.triangle_top = newValue != undefined && newValue != 'false'; break;
            case "triangle-bottom": this.triangle_bottom = newValue != undefined && newValue != 'false'; break;

            case "triangle-top-position": this.triangle_top_position = newValue; break;
            case "triangle-bottom-position": this.triangle_bottom_position = newValue; break;

            case "triangle-top-size": try {
                this.triangle_top_size = parseFloat(newValue);
            } catch {
                this.triangle_top_size = 0;
            } break;

            case "triangle-bottom-size": try {
                this.triangle_bottom_size = parseFloat(newValue);
            } catch {
                this.triangle_bottom_size = 0;
            } break;
        }

        this.render();
    }

    connectedCallback() {
        setTimeout(() => {
            this.render();
        }, 0)
    }

    degreeToRad(number) {
        return number / (180 / Math.PI);
    }

    cos(number) {
        return Math.cos(this.degreeToRad(number));
    }

    sin(number) {
        return Math.sin(this.degreeToRad(number));
    }

    tan(number) {
        return Math.tan(this.degreeToRad(number));
    }

    calculate_triangle_dimensions(angle, triangle_size) {
        // First, calculate the distance from the center of the square to a corner
        const distance_to_corner = Math.sqrt(Math.pow(triangle_size, 2) / 2);

        // Next, calculate the height from the point on the main line, to the connecting corner.
        const bottom_height = this.tan(angle) * distance_to_corner;

        const top_height = distance_to_corner - bottom_height;

        let diagonal_large = distance_to_corner / this.cos(angle);

        const diagonal_small = (top_height / this.sin(135 - (90 - angle))) * this.sin(45);

        let factor = diagonal_small / diagonal_large;

        return {
            horizontal_long: distance_to_corner,
            vertical_long: bottom_height,
            horizontal_short: distance_to_corner * factor,
            vertical_short: bottom_height * factor
        }
    }

    process_side(top, affected_children) {
        const width = this.getBoundingClientRect().width;
        const not_mobile = width > this.mobile_landscape_max_width;

        let angle = not_mobile ? (top ? this.angle_top : this.angle_bottom) : 0;
        const triangle = (top && this.triangle_top && this.triangle_top_size > 0) || (!top && this.triangle_bottom && this.triangle_bottom_size > 0);
        const triangle_size = not_mobile ? (top ? this.triangle_top_size : this.triangle_bottom_size) : 45;

        let position = not_mobile ? (() => {
            let value = top ? this.triangle_top_position : this.triangle_bottom_position;
            const is_px = value.includes("px");
            value = value.replace("px", "");
            return is_px ? parseFloat(value) / width : parseFloat(value);
        })() : ".5";

        let normal = angle >= 0;
        angle = Math.abs(angle);

        const height = this.tan(angle) * width;

        let triangle_result = ``;
        let padding = 0;

        if (!top) position = 1 - position;

        if (triangle) {
            const triangle_dimensions = this.calculate_triangle_dimensions(angle, triangle_size);

            const actual_top_position = width * position;

            const actual_top_y = height - (height * (normal ? position : 1 - position));

            const triangle_top_y = actual_top_y - (triangle_dimensions.horizontal_long - triangle_dimensions.vertical_long);

            padding = Math.max(0, -triangle_top_y);

            if (top) this.style.setProperty('--angled-padding-top', `${height + padding}px`);
            else this.style.setProperty('--angled-padding-bottom', `${height + padding}px`);

            let coordinates = [
                [actual_top_position - triangle_dimensions.horizontal_long, actual_top_y + triangle_dimensions.vertical_long + padding],
                [actual_top_position, actual_top_y - (triangle_dimensions.horizontal_long - triangle_dimensions.vertical_long) + padding],
                [actual_top_position + triangle_dimensions.horizontal_short, actual_top_y - triangle_dimensions.vertical_short + padding]
            ];

            if (!normal) {
                coordinates = [
                    [actual_top_position - triangle_dimensions.horizontal_short, actual_top_y - triangle_dimensions.vertical_short + padding],
                    [actual_top_position, actual_top_y - (triangle_dimensions.horizontal_long - triangle_dimensions.vertical_long) + padding],
                    [actual_top_position + triangle_dimensions.horizontal_long, actual_top_y + triangle_dimensions.vertical_short + padding]
                ];
            }

            coordinates.forEach(coordinate => {
                if (top) triangle_result += `${coordinate[0]}px ${coordinate[1]}px,`;
                else triangle_result += `calc(100% - ${coordinate[0]}px) calc(100% - ${coordinate[1]}px),`;
            })
        }

        if (top) {
            affected_children.forEach(child => {
                child.style.marginTop = `${height + padding}px`;
            })

            return `
            0% ${normal ? height + padding : padding}px,
            ${triangle_result}
            100% ${normal ? padding : height + padding}px,
        `;
        } else {
            affected_children.forEach(child => {
                child.style.marginBottom = `${height + padding}px`;
            })

            return `
            100% calc(100% - ${normal ? height + padding : padding}px),
            ${triangle_result}
            0% calc(100% - ${normal ? padding : height + padding}px) 
        `;
        }
    }

    render() {
        const top_side = this.process_side(true, []);
        const bottom_side = this.process_side(false, []);

        this.style.clipPath = `polygon(
                            ${top_side}
                            ${bottom_side}
                        )`;
    }
}

// Define the custom element
customElements.define('angled-element', AngledElement);