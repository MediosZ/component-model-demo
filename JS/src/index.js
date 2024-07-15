import { marked } from "marked";

// console.log(marked.parse('# Marked in the browser\n\nRendered by **marked**.'))

export const render = {
    "render": function (s) {
        return marked.parse(s)
    }
}

