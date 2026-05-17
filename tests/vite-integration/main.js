import { render, kinds, themes } from '@iyulab/declart-web';

const toml = `kind = "flow"

[[items]]
label = "Start"

[[items]]
label = "Process"

[[items]]
label = "End"
`;

try {
  const svg = render(toml);
  const parser = new DOMParser();
  const doc = parser.parseFromString(svg, 'image/svg+xml');
  document.getElementById('output').appendChild(doc.documentElement);
  document.getElementById('status').textContent =
    `OK — SVG length: ${svg.length}, kinds: ${kinds().join(', ')}, themes: ${themes().join(', ')}`;
} catch (err) {
  document.getElementById('status').textContent = `ERROR: ${err.message}`;
}
