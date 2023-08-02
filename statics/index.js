import { h, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

// Initialize htm with Preact
const html = htm.bind(h);

function App(props) {
    const evtSource = new EventSource("sse");

    evtSource.onmessage = ({ data }) => {
        console.log(data);
    }


    return html`<h1>Hello ${props.name}!</h1>`;
}

render(html`<${App} id="app" />`, document.body);